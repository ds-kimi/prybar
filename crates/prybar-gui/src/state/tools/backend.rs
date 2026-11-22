//! The real calls behind the tool screens, as plain function pointers
//! so a test can replace any one of them.

use std::path::Path;
use std::sync::atomic::AtomicBool;

use prybar_service::tools::{self, Game};
use prybar_service::ServiceError;
use prybar_tools::compile::CompileReport;
use prybar_workshop::addon::AddonCheck;
use prybar_workshop::spec::ItemSpec;
use prybar_workshop::{ListedItem, Progress, PublishReport, SteamStatus};

pub type CompileFn =
    fn(&Game, &Path, &mut dyn FnMut(&str), &AtomicBool) -> Result<CompileReport, String>;
pub type PublishFn =
    fn(&ItemSpec, bool, &mut dyn FnMut(&Progress)) -> Result<PublishReport, String>;
/// `true` detects installs through Steam and saves the merged list.
pub type GamesFn = fn(bool) -> Result<Vec<Game>, String>;
pub type SaveFn = fn(Game) -> Result<Vec<Game>, String>;
pub type StatusFn = fn(u32) -> SteamStatus;
pub type ItemsFn = fn(u32) -> Result<Vec<ListedItem>, String>;
pub type AddonFn = fn(&Path) -> Result<AddonCheck, String>;

#[derive(Clone, Copy)]
pub struct Backend {
    pub compile: CompileFn,
    pub publish: PublishFn,
    pub games: GamesFn,
    pub save: SaveFn,
    pub status: StatusFn,
    pub items: ItemsFn,
    pub addon: AddonFn,
}

impl Default for Backend {
    fn default() -> Self {
        Backend {
            compile,
            publish: |spec, dry, progress| {
                tools::publish(spec, dry, progress).map_err(|e| explain(&e))
            },
            games: |detect| {
                let r = tools::games(detect, detect).map_err(|e| e.to_string())?;
                Ok(r.games)
            },
            save: |game| Ok(tools::config_set(game).map_err(|e| e.to_string())?.games),
            status: tools::steam_status,
            items: |app| tools::workshop_list(app).map_err(|e| explain(&e)),
            addon: |dir| tools::addon_check(dir).map_err(|e| e.to_string()),
        }
    }
}

/// Says exactly what is missing before studiomdl is even tried, since
/// a bare "cannot start" helps nobody on a Mac.
fn compile(
    game: &Game,
    qc: &Path,
    on_line: &mut dyn FnMut(&str),
    cancel: &AtomicBool,
) -> Result<CompileReport, String> {
    if game.studiomdl.is_none() {
        return Err(format!(
            "{} has no studiomdl path. Set it in Settings: it is studiomdl.exe in the game's bin folder.",
            game.name
        ));
    }
    if !cfg!(windows) && !wine_found() {
        return Err(
            "studiomdl is a Windows program and no Wine was found. Install Wine \
                    (brew install --cask wine-stable) or set PRYBAR_WINE to a Proton-style \
                    runner, then restart prybar."
                .into(),
        );
    }
    tools::compile_streamed(&game.name, qc, on_line, cancel).map_err(|e| e.to_string())
}

/// `PRYBAR_WINE` if set, else `wine` somewhere on PATH, the same lookup
/// the runner does when it starts the tool.
fn wine_found() -> bool {
    if let Some(wine) = std::env::var_os("PRYBAR_WINE") {
        let path = Path::new(&wine);
        return path.is_file() || !path.is_absolute();
    }
    std::env::var_os("PATH")
        .is_some_and(|paths| std::env::split_paths(&paths).any(|dir| dir.join("wine").is_file()))
}

/// Steam problems in words that say what to do next.
pub fn explain(e: &ServiceError) -> String {
    let text = e.to_string();
    if text.contains("`workshop` feature") {
        return "This build has no Steam Workshop support. Dry run still checks everything; \
                for uploads build prybar-gui with --features workshop."
            .into();
    }
    if text.contains("init failed") {
        return "Steam is not running or no one is logged in. Start Steam, log in, then publish again."
            .into();
    }
    text
}
