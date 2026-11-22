//! `POST /v1/event`: one JSON event, validated, counted, answered `204`.

use std::net::SocketAddr;
use std::time::{Duration, Instant};

use axum::extract::{ConnectInfo, Request};
use axum::http::StatusCode;
use prybar_telemetry::Event;

use crate::app::{State, PER_MINUTE};

/// Largest accepted body; a full crash report is well under this.
const MAX_BODY: usize = 16 * 1024;

pub async fn handle(
    axum::extract::State(state): axum::extract::State<State>,
    req: Request,
) -> StatusCode {
    let addr = req
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|c| c.0.ip());
    if addr.is_some_and(|ip| !allowed(&state, ip)) {
        return StatusCode::TOO_MANY_REQUESTS;
    }
    // Headers are never read past this point and never stored.
    let Ok(body) = axum::body::to_bytes(req.into_body(), MAX_BODY).await else {
        return StatusCode::PAYLOAD_TOO_LARGE;
    };
    let Some(event) = serde_json::from_slice::<Event>(&body)
        .ok()
        .filter(Event::valid)
    else {
        return StatusCode::BAD_REQUEST;
    };
    let Ok(conn) = state.db.lock() else {
        return StatusCode::INTERNAL_SERVER_ERROR;
    };
    match crate::db::record(&conn, &crate::day::today(), &event) {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn allowed(state: &State, ip: std::net::IpAddr) -> bool {
    let Ok(mut lim) = state.limiter.lock() else {
        return false;
    };
    if lim.window.elapsed() >= Duration::from_secs(60) {
        lim.window = Instant::now();
        lim.counts.clear();
    }
    let n = lim.counts.entry(ip).or_insert(0);
    *n += 1;
    *n <= PER_MINUTE
}
