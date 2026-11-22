use crate::guard;
use crate::params::{ConfigSetArgs, GamesArgs};
use crate::reply::{blocking, failure};
use crate::server::Prybar;
use prybar_service::tools::{self, Game, Packer};
use prybar_service::{Result, ServiceError};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router};

#[tool_router(router = config_router, vis = "pub(crate)")]
impl Prybar {
    #[tool(
        description = "List the configured games (name, game_dir, studiomdl, hlmv, packer). With `detect`, also add Steam installs found on this machine; with `save`, write the result to the config file.",
        annotations(
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn games_list(&self, Parameters(a): Parameters<GamesArgs>) -> CallToolResult {
        blocking(move || tools::games(a.detect, a.save)).await
    }

    #[tool(
        description = "Show the prybar config file path and every configured game.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn config_get(&self) -> CallToolResult {
        blocking(tools::config_get).await
    }

    /// No tool paths here: compile_model and view_model run them, and
    /// unpack_package can write a file anywhere, so an agent setting them
    /// could run a program it wrote. They come from detection or the CLI.
    #[tool(
        description = "Add or replace one game in the config (same name, case-insensitive; its detected studiomdl/hlmv paths are kept), or remove it with `remove: true`. Saves the config file and returns the new game list.",
        annotations(
            destructive_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn config_set(&self, Parameters(a): Parameters<ConfigSetArgs>) -> CallToolResult {
        if a.remove {
            return blocking(move || tools::config_remove(&a.name)).await;
        }
        match game(&a) {
            Ok(g) => blocking(move || tools::config_set(g)).await,
            Err(e) => failure(&e),
        }
    }
}

fn game(a: &ConfigSetArgs) -> Result<Game> {
    let dir = a
        .game_dir
        .as_deref()
        .ok_or_else(|| ServiceError::Usage("game_dir is required unless remove is true".into()))?;
    let packer = match a.packer.as_deref() {
        None => None,
        Some("gma") => Some(Packer::Gma),
        Some("vpk") => Some(Packer::Vpk),
        Some(other) => {
            return Err(ServiceError::Usage(format!(
                "packer: `{other}` is not gma or vpk"
            )))
        }
    };
    Ok(Game {
        name: a.name.clone(),
        game_dir: guard::absolute("game_dir", dir)?,
        studiomdl: None,
        hlmv: None,
        packer,
    })
}
