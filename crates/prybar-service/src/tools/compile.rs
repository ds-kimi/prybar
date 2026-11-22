use super::{games::game, tool_error};
use crate::{Result, ServiceError};
use prybar_tools::compile::CompileReport;
use prybar_tools::runner::Runner;
use prybar_tools::ToolError;
use std::path::Path;

/// Runs the game's studiomdl on `qc`. A failed compile is still `Ok`:
/// the report carries the errors an agent needs to fix the QC.
pub fn compile(game_name: &str, qc: &Path) -> Result<CompileReport> {
    if !qc.is_file() {
        return Err(ServiceError::NotFound(qc.display().to_string()));
    }
    let g = game(game_name)?;
    let studiomdl = g.studiomdl.ok_or_else(|| {
        tool_error(ToolError::MissingTool {
            game: g.name.clone(),
            tool: "studiomdl",
        })
    })?;
    prybar_tools::compile::compile(&Runner::for_host(), &studiomdl, &g.game_dir, qc)
        .map_err(tool_error)
}
