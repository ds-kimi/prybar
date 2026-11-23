use prybar_tools::config;
use prybar_tools::paths::find_game_dir;
use prybar_workshop::games::{Unsupported, WorkshopGame, UNSUPPORTED, WORKSHOP_GAMES};
use serde::Serialize;

use crate::{Result, ServiceError};

/// What the publish screen and `publish --games` show: the supported
/// games found on this machine, and every game prybar knows with why it
/// cannot be published to.
#[derive(Debug, Clone, Serialize)]
pub struct WorkshopGames {
    pub detected: Vec<WorkshopGame>,
    pub supported: Vec<WorkshopGame>,
    pub unsupported: Vec<Unsupported>,
}

/// A game counts as detected when it is in the games config or installed
/// in a Steam library, so a hand-configured install is found too.
pub fn workshop_games() -> WorkshopGames {
    let cfg = config::load_default().unwrap_or_default();
    let found = |g: &&WorkshopGame| {
        cfg.game(g.id).is_some_and(|c| c.game_dir.is_dir()) || find_game_dir(g.install).is_some()
    };
    WorkshopGames {
        detected: WORKSHOP_GAMES.iter().filter(found).copied().collect(),
        supported: WORKSHOP_GAMES.to_vec(),
        unsupported: UNSUPPORTED.to_vec(),
    }
}

/// `--game` value to app id, with the reason when the game is known but
/// has no Workshop prybar can upload to.
pub fn workshop_app_id(game: &str) -> Result<u32> {
    if let Some(g) = prybar_workshop::games::by_id(game) {
        return Ok(g.app_id);
    }
    let why = match prybar_workshop::games::unsupported(game) {
        Some(u) => format!("{} {}", u.name, u.reason),
        None => {
            let ids: Vec<&str> = WORKSHOP_GAMES.iter().map(|g| g.id).collect();
            format!("unknown Workshop game `{game}`; use {}", ids.join(" or "))
        }
    };
    Err(ServiceError::Usage(why))
}
