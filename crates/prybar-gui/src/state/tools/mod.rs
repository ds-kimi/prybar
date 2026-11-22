//! Compile, publish and the games config: the screens that drive
//! external tools. Each call goes through [`Backend`] so tests swap in
//! fakes and never start studiomdl or talk to Steam.

mod backend;
mod compile;
mod games;
mod publish;
mod result;
mod steam;

pub use backend::Backend;
#[cfg(test)]
pub use compile::LineKind;
pub use compile::{CompileOutcome, CompileRun};
pub use publish::{PublishForm, PublishRun};
pub use steam::SteamView;

use prybar_service::tools::Game;

#[derive(Default)]
pub struct Tools {
    pub backend: Backend,
    /// Games from the config; read the first time a screen needs them.
    pub games: Vec<Game>,
    pub games_loaded: bool,
    /// Index into `games` for the compile screen.
    pub game: usize,
    /// The game being edited on the settings screen.
    pub editing: Option<usize>,
    pub compile: CompileRun,
    pub publish: PublishRun,
    pub steam: SteamView,
    /// Set once the publish screen asked Steam the first time.
    pub steam_started: bool,
}
