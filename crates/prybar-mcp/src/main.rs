//! `prybar-mcp`: the prybar operations as MCP tools over stdio, for AI
//! agents. Logs go to stderr because stdout carries the JSON-RPC stream.

use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let server = prybar_mcp::Prybar::new();
    let running = match server.serve(rmcp::transport::stdio()).await {
        Ok(r) => r,
        Err(e) => return fail(&e),
    };
    match running.waiting().await {
        Ok(_) => std::process::ExitCode::SUCCESS,
        Err(e) => fail(&e),
    }
}

fn fail(e: &dyn std::fmt::Display) -> std::process::ExitCode {
    eprintln!("prybar-mcp: {e}");
    std::process::ExitCode::FAILURE
}
