//! `GET /stats`: a read-only page for the owner, behind the env token.

use axum::extract::{Request, State as Extract};
use axum::http::{header, StatusCode};
use axum::response::{Html, IntoResponse, Response};

use crate::app::State;
use crate::db::Rows;

pub async fn handle(Extract(state): Extract<State>, req: Request) -> Response {
    let Some(token) = state.token.as_deref() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let given = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("");
    if !same(given.as_bytes(), token.as_bytes()) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let read = match state.db.lock() {
        Ok(conn) => crate::db::read(&conn),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    match read {
        Ok((daily, crashes)) => Html(page(&daily, &crashes)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Constant time, so the token cannot be guessed byte by byte.
fn same(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn page(daily: &Rows, crashes: &Rows) -> String {
    let table = |head: &str, rows: &[Vec<String>]| {
        let body: String = rows
            .iter()
            .map(|r| {
                format!(
                    "<tr>{}</tr>",
                    r.iter()
                        .map(|c| format!("<td>{}</td>", escape(c)))
                        .collect::<String>()
                )
            })
            .collect();
        format!("<table><tr>{head}</tr>{body}</table>")
    };
    format!(
        "<!doctype html><meta charset=utf-8><title>prybar stats</title>\
         <style>body{{font:14px system-ui;margin:2em}}td,th{{padding:2px 8px;text-align:left}}</style>\
         <h1>Daily events (estimate: starts, not people)</h1>{}<h1>Recent crashes</h1>{}",
        table("<th>Day<th>Event<th>Version<th>OS<th>Arch<th>Count", daily),
        table("<th>Day<th>Version<th>OS<th>Arch<th>Message<th>Frames", crashes),
    )
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
