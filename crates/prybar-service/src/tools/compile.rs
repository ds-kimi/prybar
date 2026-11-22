use super::{games::game, tool_error};
use crate::{Result, ServiceError};
use prybar_tools::compile::CompileReport;
use prybar_tools::config::Game;
use prybar_tools::runner::Runner;
use prybar_tools::ToolError;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

/// Runs the game's studiomdl on `qc`. A failed compile is still `Ok`:
/// the report carries the errors an agent needs to fix the QC.
pub fn compile(game_name: &str, qc: &Path) -> Result<CompileReport> {
    let (g, studiomdl) = studiomdl_for(game_name, qc)?;
    prybar_tools::compile::compile(&Runner::for_host(), &studiomdl, &g.game_dir, qc)
        .map_err(tool_error)
}

/// Like [`compile`], with each log line handed to `on_line` as it is
/// printed; setting `cancel` stops the compile.
pub fn compile_streamed(
    game_name: &str,
    qc: &Path,
    on_line: &mut dyn FnMut(&str),
    cancel: &AtomicBool,
) -> Result<CompileReport> {
    let (g, studiomdl) = studiomdl_for(game_name, qc)?;
    let paths = (studiomdl.as_path(), g.game_dir.as_path(), qc);
    prybar_tools::compile::compile_streamed(&Runner::for_host(), paths, on_line, cancel)
        .map_err(tool_error)
}

fn studiomdl_for(game_name: &str, qc: &Path) -> Result<(Game, PathBuf)> {
    if !qc.is_file() {
        return Err(ServiceError::NotFound(qc.display().to_string()));
    }
    let g = game(game_name)?;
    let studiomdl = g.studiomdl.clone().ok_or_else(|| {
        tool_error(ToolError::MissingTool {
            game: g.name.clone(),
            tool: "studiomdl",
        })
    })?;
    Ok((g, studiomdl))
}
