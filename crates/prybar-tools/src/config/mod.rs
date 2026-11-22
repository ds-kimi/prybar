//! Game setup: which games exist, where their content lives and which
//! tools compile and view their models. Stored as TOML in the user
//! config dir so the CLI and the MCP server share it.

mod detect;
mod known;
mod model;
mod store;

pub use detect::{detect_games, game_from_install};
pub use known::{KnownGame, KNOWN_GAMES};
pub use model::{Config, Game, Packer};
pub use store::{config_path, load, load_default, save};
