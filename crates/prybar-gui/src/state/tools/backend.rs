//! The real calls behind the tool screens, as plain function pointers
//! so a test can replace any one of them.

use std::path::Path;
use std::sync::atomic::AtomicBool;

use prybar_service::tools::{self, Game};
use prybar_service::ServiceError;
use prybar_tools::compile::CompileReport;
use prybar_workshop::addon::AddonCheck;
use prybar_workshop::changelog::Log;
use prybar_workshop::http::Failure;
use prybar_workshop::spec::ItemSpec;
use prybar_workshop::{Avatar, ListedItem, Progress, PublishReport, SteamStatus};

pub type CompileFn =
    fn(&Game, &Path, &mut dyn FnMut(&str), &AtomicBool) -> Result<CompileReport, String>;
/// Game, inputs, options, the "started" callback and the cancel flag.
pub type BatchFn = fn(
    &Game,
    &[String],
    &tools::CompileOptions,
    &(dyn Fn(&str) + Sync),
    &AtomicBool,
) -> Result<tools::CompileBatchReport, String>;
pub type PublishFn =
    fn(&ItemSpec, bool, &mut dyn FnMut(&Progress)) -> Result<PublishReport, String>;
/// `true` detects installs through Steam and saves the merged list.
pub type GamesFn = fn(bool) -> Result<Vec<Game>, String>;
pub type SaveFn = fn(Game) -> Result<Vec<Game>, String>;
pub type StatusFn = fn(u32) -> SteamStatus;
pub type ItemsFn = fn(u32) -> Result<Vec<ListedItem>, String>;
pub type AddonFn = fn(&Path) -> Result<AddonCheck, String>;
pub type AvatarFn = fn(u32) -> Option<Avatar>;
/// A preview URL to a cached image file.
pub type ThumbFn = fn(&str) -> Result<std::path::PathBuf, String>;
pub type HistoryFn = fn(u64) -> Result<Log, Failure>;
/// Where stats snapshots and folder links are kept.
pub type DataFn = fn() -> std::path::PathBuf;
/// Supported Workshop games and which are installed here.
pub type TargetsFn = fn() -> tools::WorkshopGames;

#[derive(Clone, Copy)]
pub struct Backend {
    pub compile: CompileFn,
    pub compile_batch: BatchFn,
    pub publish: PublishFn,
    pub games: GamesFn,
    pub save: SaveFn,
    pub status: StatusFn,
    pub items: ItemsFn,
    pub addon: AddonFn,
    pub avatar: AvatarFn,
    pub thumb: ThumbFn,
    pub history: HistoryFn,
    pub data_dir: DataFn,
    pub targets: TargetsFn,
}

impl Default for Backend {
    fn default() -> Self {
        Backend {
            compile,
            compile_batch: crate::state::batch::compile_batch,
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
            avatar: tools::steam_avatar,
            thumb: super::data::default_thumb,
            history: |id| {
                prybar_workshop::changelog::cached(id, &super::data::data_dir().join("changelogs"))
            },
            data_dir: super::data::data_dir,
            targets: tools::workshop_games,
        }
    }
}

fn compile(
    game: &Game,
    qc: &Path,
    on_line: &mut dyn FnMut(&str),
    cancel: &AtomicBool,
) -> Result<CompileReport, String> {
    crate::state::batch::precheck(game)?;
    tools::compile_streamed(&game.name, qc, on_line, cancel).map_err(|e| e.to_string())
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
