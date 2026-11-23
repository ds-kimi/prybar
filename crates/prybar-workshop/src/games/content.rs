use std::path::Path;

use super::{by_app, Content, WorkshopGame};
use crate::WorkshopError;

/// Most map folders keep the `.bsp` in `maps/`; a flat folder works too.
const MAP_DIRS: [&str; 2] = ["", "maps"];

/// Unknown app ids are refused with the list of supported ones, so a
/// typo cannot upload to the wrong game.
pub fn supported(app_id: u32) -> Result<&'static WorkshopGame, WorkshopError> {
    by_app(app_id).ok_or_else(|| {
        let known: Vec<String> = super::WORKSHOP_GAMES
            .iter()
            .map(|g| format!("{} ({})", g.app_id, g.id))
            .collect();
        let why = format!("not a supported Workshop game; use {}", known.join(" or "));
        WorkshopError::invalid("app_id", why)
    })
}

/// Checks a folder against the rules of a game that uploads folders.
pub fn check_game_content(app_id: u32, path: &Path) -> Result<(), WorkshopError> {
    let game = supported(app_id)?;
    match game.content {
        Content::Gma => Ok(()),
        Content::Maps if has_map(path) => Ok(()),
        Content::Maps => Err(WorkshopError::invalid(
            "content",
            format!("must be {}", game.rules),
        )),
    }
}

fn has_map(dir: &Path) -> bool {
    MAP_DIRS.iter().any(|sub| {
        std::fs::read_dir(dir.join(sub)).is_ok_and(|rd| {
            rd.flatten().any(|e| {
                let p = e.path();
                p.is_file()
                    && p.extension()
                        .is_some_and(|x| x.eq_ignore_ascii_case("bsp"))
            })
        })
    })
}
