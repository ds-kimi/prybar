use crate::guard;
use crate::params::{CompileArgs, PublishArgs, ViewArgs};
use crate::reply::{blocking, failure};
use crate::server::Prybar;
use prybar_service::{tools, Result, ServiceError};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router};

#[tool_router(router = external_router, vis = "pub(crate)")]
impl Prybar {
    #[tool(
        description = "Compile a QC file with the configured game's studiomdl. A compile that runs but fails still returns a result with `success: false`, the error lines and the full log, so the QC can be fixed and compiled again. `outputs` lists the files studiomdl wrote (.mdl, .vvd, .vtx, .phy, .ani, .cvvd).",
        annotations(
            destructive_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn compile_model(&self, Parameters(a): Parameters<CompileArgs>) -> CallToolResult {
        match guard::absolute("qc", &a.qc) {
            Ok(qc) => blocking(move || tools::compile(&a.game, &qc)).await,
            Err(e) => failure(&e),
        }
    }

    #[tool(
        description = "Open a model in the configured game's hlmv (Half-Life Model Viewer) for a human to look at. Returns at once with the viewer's process id.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            open_world_hint = false
        )
    )]
    async fn view_model(&self, Parameters(a): Parameters<ViewArgs>) -> CallToolResult {
        match guard::absolute("model", &a.model) {
            Ok(m) => blocking(move || tools::view(&a.game, &m)).await,
            Err(e) => failure(&e),
        }
    }

    #[tool(
        description = "Create or update a Steam Workshop item from a JSON spec file, list the user's items with `game` (or `list_app_id`), or list supported and installed games with `list_games`. Supported games: Garry's Mod (`garrysmod`, 4000) and Team Fortress 2 maps (`tf2`, 440); the description is Steam BBCode. Needs a prybar build with the `workshop` feature and a running, signed-in Steam client. Use `dry_run` first: a real publish is visible on Steam.",
        annotations(
            destructive_hint = true,
            idempotent_hint = false,
            open_world_hint = true
        )
    )]
    async fn publish_workshop(&self, Parameters(a): Parameters<PublishArgs>) -> CallToolResult {
        if a.list_games {
            return blocking(|| Ok(tools::workshop_games())).await;
        }
        if let Some(app_id) = a.list_app_id {
            return blocking(move || tools::workshop_list(app_id)).await;
        }
        let game = a.game.clone();
        match a.spec.as_deref().map(|s| guard::absolute("spec", s)) {
            Some(Ok(spec)) => blocking(move || publish(&spec, game, a.dry_run, a.confirm)).await,
            Some(Err(e)) => failure(&e),
            None => match game {
                Some(g) => {
                    blocking(move || tools::workshop_list(tools::workshop_app_id(&g)?)).await
                }
                None => failure(&ServiceError::Usage(
                    "pass `spec`, `game`, `list_app_id` or `list_games`".into(),
                )),
            },
        }
    }
}

/// Paths inside the spec are checked like tool arguments: a relative one
/// would resolve against the server's working directory.
/// A real upload also needs `confirm`, so an agent cannot publish by
/// forgetting `dry_run`.
fn publish(
    spec: &std::path::Path,
    game: Option<String>,
    dry_run: bool,
    confirm: bool,
) -> Result<prybar_workshop::PublishReport> {
    if !dry_run && !confirm {
        return Err(ServiceError::Usage(
            "a real publish needs `confirm: true`; run with `dry_run` first and ask the user"
                .into(),
        ));
    }
    let mut item = tools::load_spec(spec)?;
    if let Some(g) = game {
        item.app_id = tools::workshop_app_id(&g)?;
    }
    for (field, path) in [("preview", &item.preview), ("content", &item.content)] {
        if let Some(p) = path {
            guard::absolute(field, &p.to_string_lossy())?;
        }
    }
    tools::publish(&item, dry_run, &mut |_| {})
}
