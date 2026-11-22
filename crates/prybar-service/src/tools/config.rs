use super::tool_error;
use crate::{Result, ServiceError};
use prybar_tools::config::{self, Game};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConfigReport {
    pub config_path: PathBuf,
    pub games: Vec<Game>,
}

pub fn config_get() -> Result<ConfigReport> {
    let path = config::config_path().map_err(tool_error)?;
    let cfg = config::load(&path).map_err(tool_error)?;
    Ok(ConfigReport {
        config_path: path,
        games: cfg.games,
    })
}

/// Replaces the game of the same name (case-insensitive) or adds it, then
/// saves. Tool paths left unset keep the old entry's values, so editing a
/// detected game's folder does not lose its studiomdl and hlmv. A relative `game_dir` would resolve against whatever directory
/// the caller happens to run in, so it is refused.
pub fn config_set(mut game: Game) -> Result<ConfigReport> {
    if game.name.trim().is_empty() || !game.game_dir.is_absolute() {
        return Err(ServiceError::Usage(
            "a game needs a name and an absolute game_dir".into(),
        ));
    }
    edit(|games| {
        if let Some(old) = games
            .iter()
            .find(|g| g.name.eq_ignore_ascii_case(&game.name))
        {
            game.studiomdl = game.studiomdl.take().or_else(|| old.studiomdl.clone());
            game.hlmv = game.hlmv.take().or_else(|| old.hlmv.clone());
        }
        games.retain(|g| !g.name.eq_ignore_ascii_case(&game.name));
        games.push(game);
        true
    })
}

/// Drops the game called `name`; unknown names are a usage error so a
/// typo does not look like success.
pub fn config_remove(name: &str) -> Result<ConfigReport> {
    edit(|games| {
        let before = games.len();
        games.retain(|g| !g.name.eq_ignore_ascii_case(name));
        games.len() != before
    })
    .map_err(|e| match e {
        ServiceError::NotFound(_) => ServiceError::NotFound(format!("game {name}")),
        e => e,
    })
}

fn edit(change: impl FnOnce(&mut Vec<Game>) -> bool) -> Result<ConfigReport> {
    let path = config::config_path().map_err(tool_error)?;
    let mut cfg = config::load(&path).map_err(tool_error)?;
    if !change(&mut cfg.games) {
        return Err(ServiceError::NotFound("game".into()));
    }
    config::save(&path, &cfg).map_err(tool_error)?;
    Ok(ConfigReport {
        config_path: path,
        games: cfg.games,
    })
}
