use rmcp::handler::server::router::tool::ToolRouter;
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
                + Self::config_router(),
        }
    }
}

impl Default for Prybar {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_handler(
    router = self.tool_router,
    name = "prybar",
    instructions = "Source engine modding tools. Decompile MDL models (versions 4 to 53) to QC/SMD/VTA/VRD, list/unpack/pack VPK and GMA packages, list/unpack APK and HFS, compile with studiomdl, view with hlmv, publish to the Steam Workshop. Every path must be absolute. A model inside a package is written `<absolute package path>:<inner path>`, e.g. `C:/game/pak01_dir.vpk:models/props/crate.mdl`. Results are JSON; failures carry `error`, `kind` and the CLI exit `code`. Configure games with games_list (detect=true) or config_set before compile_model or view_model."
)]
impl ServerHandler for Prybar {}
