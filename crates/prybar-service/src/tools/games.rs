use super::tool_error;
use crate::Result;
use prybar_tools::config::{self, Game};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GamesReport {
    pub config_path: PathBuf,
    pub games: Vec<Game>,
    /// Games added by detection in this call.
    pub detected: usize,
    pub saved: bool,
}

/// Configured games; `detect` merges installs found through Steam, and
/// `save` writes the merged list back (hand edits are kept).
pub fn games(detect: bool, save: bool) -> Result<GamesReport> {
    let path = config::config_path().map_err(tool_error)?;
    let mut cfg = config::load(&path).map_err(tool_error)?;
    let before = cfg.games.len();
    if detect {
        cfg.merge(config::detect_games());
    }
    if save {
        config::save(&path, &cfg).map_err(tool_error)?;
    }
    Ok(GamesReport {
        config_path: path,
        detected: cfg.games.len() - before,
        games: cfg.games,
        saved: save,
    })
}

/// The configured game called `name` (case-insensitive).
pub(crate) fn game(name: &str) -> Result<Game> {
    let cfg = config::load_default().map_err(tool_error)?;
    cfg.game(name)
        .cloned()
        .ok_or_else(|| tool_error(prybar_tools::ToolError::UnknownGame(name.to_owned())))
}
