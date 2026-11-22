//! Per-OS resolution of Steam and game folders. Env vars win over
//! detection so CI and unusual installs never depend on guesses.

mod config_dir;
mod defaults;
mod games;
mod registry;
mod roots;
mod vdf;

pub use config_dir::user_config_dir;
pub use games::{find_game_dir, gmod_content_dir, gmod_dir};
pub use roots::{steam_libraries, steam_roots};
pub use vdf::library_paths;
