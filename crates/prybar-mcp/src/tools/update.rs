use crate::reply::blocking;
use crate::server::Prybar;
use prybar_update::{check, checks_disabled, fresh_status, Config, Status, CHECK_EVERY};
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router};
use serde::Serialize;

const CURRENT: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
struct UpdateReport {
    current: String,
    /// `null` when GitHub could not be reached.
    latest: Option<String>,
    update_available: bool,
    /// What the user runs to update; the server never replaces itself.
    command: Option<String>,
}

#[tool_router(router = update_router, vis = "pub(crate)")]
impl Prybar {
    #[tool(
        description = "Check GitHub for a newer prybar release. Returns current, latest (null if offline), update_available and the command to run (`prybar self-update`). Never installs anything: the running server is not replaced.",
        annotations(read_only_hint = true, open_world_hint = true)
    )]
    async fn check_update(&self) -> CallToolResult {
        blocking(|| Ok(report(&check(&Config::from_env(CURRENT))))).await
    }
}

fn report(status: &Status) -> UpdateReport {
    let latest = match status {
        Status::Available(r) => Some(r.version.to_string()),
        Status::UpToDate => Some(CURRENT.to_string()),
        Status::Unknown(_) => None,
    };
    let available = matches!(status, Status::Available(_));
    UpdateReport {
        current: CURRENT.into(),
        latest,
        update_available: available,
        command: available.then(|| "prybar self-update".into()),
    }
}

/// The notice for the `initialize` instructions, from the cache only.
pub fn update_notice() -> Option<String> {
    if checks_disabled() {
        return None;
    }
    match fresh_status(&Config::from_env(CURRENT), CHECK_EVERY * 7)? {
        Status::Available(r) => Some(format!(
            "Update: prybar {} is available (this server is {CURRENT}); tell the user to run `prybar self-update` and restart the MCP server.",
            r.version
        )),
        _ => None,
    }
}

/// Refreshes the cache at startup when due, off the stdio loop, so the
/// next `initialize` can mention a new release.
pub fn background_check() {
    let cfg = Config::from_env(CURRENT);
    if checks_disabled() || fresh_status(&cfg, CHECK_EVERY).is_some() {
        return;
    }
    std::thread::spawn(move || check(&cfg));
}
