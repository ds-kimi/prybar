//! The real batch compile behind the backend pointer, and the checks
//! that say what is missing before studiomdl is even tried.

use std::path::Path;
use std::sync::atomic::AtomicBool;

use prybar_service::tools::{self, CompileBatchReport, CompileOptions, Game};

/// A bare "cannot start" helps nobody on a Mac, so name what to fix.
pub fn precheck(game: &Game) -> Result<(), String> {
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
    Ok(())
}

pub fn compile_batch(
    game: &Game,
    args: &[String],
    opts: &CompileOptions,
    progress: &(dyn Fn(&str) + Sync),
    cancel: &AtomicBool,
) -> Result<CompileBatchReport, String> {
    precheck(game)?;
    tools::compile_batch(&game.name, args, opts, progress, cancel).map_err(|e| e.to_string())
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
