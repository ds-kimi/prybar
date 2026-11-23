//! Which Source games take Workshop uploads from a third-party tool, and
//! what each one accepts. Sources and the reasons for the unsupported
//! ones are in `docs/workshop-games.md`.

mod content;
mod list;

pub use content::{check_game_content, supported};
pub use list::{Content, Unsupported, WorkshopGame, UNSUPPORTED, WORKSHOP_GAMES};

/// By preset name (`garrysmod`, `tf2`) or by app id written as text, so
/// `--game 4000` works as well as `--game garrysmod`.
pub fn by_id(id: &str) -> Option<&'static WorkshopGame> {
    let id = id.trim();
    WORKSHOP_GAMES
        .iter()
        .find(|g| g.id.eq_ignore_ascii_case(id) || id.parse() == Ok(g.app_id))
}

/// The supported game behind a Steam app id.
pub fn by_app(app_id: u32) -> Option<&'static WorkshopGame> {
    WORKSHOP_GAMES.iter().find(|g| g.app_id == app_id)
}

/// Why `id` (preset name or app id) cannot be published to, if prybar
/// knows the game.
pub fn unsupported(id: &str) -> Option<&'static Unsupported> {
    let id = id.trim();
    UNSUPPORTED
        .iter()
        .find(|g| g.id.eq_ignore_ascii_case(id) || id.parse() == Ok(g.app_id))
}
