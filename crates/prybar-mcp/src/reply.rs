//! Every tool answers with structured JSON. Service failures become tool
//! errors (`isError: true`) rather than protocol errors, so the agent sees
//! the message and can fix its arguments.

use prybar_service::{ExitCode, ServiceError};
use rmcp::model::CallToolResult;
use serde::Serialize;
use serde_json::json;

pub fn reply<T: Serialize>(result: prybar_service::Result<T>) -> CallToolResult {
    match result.map(|v| serde_json::to_value(v)) {
        Ok(Ok(value)) => CallToolResult::structured(value),
        Ok(Err(e)) => failure(&ServiceError::Tool(format!("encoding the result: {e}"))),
        Err(e) => failure(&e),
    }
}

pub fn failure(e: &ServiceError) -> CallToolResult {
    let code = e.exit_code();
    CallToolResult::structured_error(json!({
        "error": e.to_string(),
        "kind": kind(code),
        "code": code as u8,
    }))
}

/// The service is synchronous and may spend seconds on disk and CPU; it
/// runs off the async executor so the stdio loop keeps answering.
pub async fn blocking<T, F>(work: F) -> CallToolResult
where
    T: Serialize + Send + 'static,
    F: FnOnce() -> prybar_service::Result<T> + Send + 'static,
{
    match tokio::task::spawn_blocking(work).await {
        Ok(result) => reply(result),
        Err(e) => failure(&ServiceError::Tool(format!("tool task failed: {e}"))),
    }
}

fn kind(code: ExitCode) -> &'static str {
    match code {
        ExitCode::Ok => "ok",
        ExitCode::Internal => "internal",
        ExitCode::Usage => "usage",
        ExitCode::NotFound => "not_found",
        ExitCode::InvalidInput => "invalid_input",
        ExitCode::Io => "io",
        ExitCode::Tool => "tool",
        ExitCode::Partial => "partial",
    }
}
