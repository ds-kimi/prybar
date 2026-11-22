use super::{games::game, tool_error};
use crate::{Result, ServiceError};
use prybar_tools::runner::Runner;
use prybar_tools::ToolError;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ViewReport {
    pub pid: u32,
}

/// Opens `mdl` in the game's hlmv and returns at once.
pub fn view(game_name: &str, mdl: &Path) -> Result<ViewReport> {
    if !mdl.is_file() {
        return Err(ServiceError::NotFound(mdl.display().to_string()));
    }
    let g = game(game_name)?;
    let hlmv = g.hlmv.ok_or_else(|| {
        tool_error(ToolError::MissingTool {
            game: g.name.clone(),
            tool: "hlmv",
        })
    })?;
    let pid = prybar_tools::view::view(&Runner::for_host(), &hlmv, &g.game_dir, mdl)
        .map_err(tool_error)?;
    Ok(ViewReport { pid })
}
