use rmcp::model::{CallToolRequestParams, ClientConfig};
use rmcp::service::RunningService;
use rmcp::{ClientHandler, RoleClient, ServiceExt};
use serde_json::Value;
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Debug, Clone, Default)]
pub struct Agent;

impl ClientHandler for Agent {
    fn get_info(&self) -> ClientConfig {
        ClientConfig::default()
    }
}

pub type Client = RunningService<RoleClient, Agent>;

/// Server and client joined by an in-memory pipe. Every test binary run
/// shares one private config file so the user's real one is never read.
pub async fn connect() -> Client {
    static CONFIG: OnceLock<PathBuf> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap().keep();
        let path = dir.join("config.toml");
        std::env::set_var("PRYBAR_CONFIG", &path);
        path
    });
    let (server_io, client_io) = tokio::io::duplex(1 << 16);
    tokio::spawn(async move {
        if let Ok(running) = prybar_mcp::Prybar::new().serve(server_io).await {
            let _ = running.waiting().await;
        }
    });
    Agent.serve(client_io).await.unwrap()
}

/// Calls `tool` and returns `(is_error, structured content)`. Arguments
/// that do not match the schema are rejected by rmcp before our code
/// runs, as a plain text error: those come back as `{"kind": "schema"}`.
pub async fn call(client: &Client, tool: &str, args: Value) -> (bool, Value) {
    let params = CallToolRequestParams::new(tool.to_owned())
        .with_arguments(args.as_object().cloned().unwrap_or_default());
    let r = client.call_tool(params).await.unwrap();
    let is_error = r.is_error == Some(true);
    match r.structured_content {
        Some(value) => (is_error, value),
        None => (is_error, serde_json::json!({ "kind": "schema" })),
    }
}
