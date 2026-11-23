use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::{Implementation, ServerCapabilities, ServerConfig};
use rmcp::{tool_handler, ServerHandler};

/// Stateless: every tool reads its inputs from disk, so one instance can
/// serve concurrent calls.
#[derive(Debug, Clone)]
pub struct Prybar {
    tool_router: ToolRouter<Self>,
}

impl Prybar {
    pub fn new() -> Self {
        Self {
            tool_router: Self::model_router()
                + Self::package_router()
                + Self::external_router()
                + Self::config_router()
                + Self::update_router()
                + Self::workshop_router()
                + Self::check_router(),
        }
    }
}

impl Default for Prybar {
    fn default() -> Self {
        Self::new()
    }
}

const INSTRUCTIONS: &str = "Source engine modding tools. Decompile MDL models (versions 4 to 53) to QC/SMD/VTA/VRD, list/unpack/pack VPK and GMA packages, list/unpack APK and HFS, compile with studiomdl, view with hlmv, publish to the Steam Workshop. Every path must be absolute. A model inside a package is written `<absolute package path>:<inner path>`, e.g. `C:/game/pak01_dir.vpk:models/props/crate.mdl`. Results are JSON; failures carry `error`, `kind` and the CLI exit `code`. Configure games with games_list (detect=true) or config_set before compile_model or view_model.";

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Prybar {
    /// Written by hand so the instructions can carry the update notice from
    /// the cached check; `initialize` never waits on the network.
    fn get_info(&self) -> ServerConfig {
        let mut text = INSTRUCTIONS.to_string();
        if let Some(line) = crate::tools::update_notice() {
            text = format!("{text} {line}");
        }
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("prybar", env!("CARGO_PKG_VERSION")))
            .with_instructions(text)
    }
}
