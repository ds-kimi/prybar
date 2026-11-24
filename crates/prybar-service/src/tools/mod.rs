//! External tools and settings: game config, studiomdl, hlmv, Workshop.

mod compile;
mod config;
mod games;
mod publish;
mod spec;
mod targets;
mod view;
mod workshop;

pub use compile::{
    compile, compile_batch, compile_streamed, qc_files, CompileBatchReport, CompileItem,
    CompileOptions, ItemStatus,
};
pub use config::{config_get, config_remove, config_set, ConfigReport};
pub use games::{games, GamesReport};
pub use prybar_tools::config::{Game, Packer};
pub use prybar_tools::paths::steam_libraries;
pub use publish::{publish, workshop_list};
pub use spec::load_spec;
pub use targets::{workshop_app_id, workshop_games, WorkshopGames};
pub use view::{view, ViewReport};
pub use workshop::{addon_check, steam_avatar, steam_status};

use crate::ServiceError;
use prybar_tools::ToolError;

/// Config and lookup problems are the caller's to fix; a tool that will
/// not start is an external failure.
fn tool_error(e: ToolError) -> ServiceError {
    match e {
        ToolError::UnknownGame(_) | ToolError::MissingTool { .. } => {
            ServiceError::Usage(e.to_string())
        }
        ToolError::Config { .. } => ServiceError::invalid("config", e),
        ToolError::Io { path, source } => ServiceError::Io { path, source },
        _ => ServiceError::Tool(e.to_string()),
    }
}
