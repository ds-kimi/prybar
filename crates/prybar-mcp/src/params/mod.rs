//! Tool arguments. Field docs become the JSON schema descriptions the
//! agent reads, so they say what to pass, not how it is used inside.

mod config;
mod external;
mod model;
mod package;

pub use config::{ConfigSetArgs, GamesArgs};
pub use external::{CompileArgs, PublishArgs, ViewArgs};
pub use model::{DecompileArgs, InspectArgs};
pub use package::{ListArgs, PackArgs, UnpackArgs};
