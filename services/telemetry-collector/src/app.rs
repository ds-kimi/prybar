//! Shared state and routes.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use axum::routing::{get, post};
use axum::Router;
use rusqlite::Connection;

/// Requests per address per minute before `429`.
pub const PER_MINUTE: u32 = 30;

/// Request counts per address for the current minute. Memory only, wiped
/// every minute, never written or logged: it exists to refuse floods.
pub struct Limiter {
    pub window: Instant,
    pub counts: HashMap<IpAddr, u32>,
}

#[derive(Clone)]
pub struct State {
    pub db: Arc<Mutex<Connection>>,
    pub token: Option<Arc<str>>,
    pub limiter: Arc<Mutex<Limiter>>,
}

impl State {
    pub fn new(db: Connection, token: Option<String>) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            token: token.map(Into::into),
            limiter: Arc::new(Mutex::new(Limiter {
                window: Instant::now(),
                counts: HashMap::new(),
            })),
        }
    }
}

pub fn router(state: State) -> Router {
    Router::new()
        .route("/v1/event", post(crate::ingest::handle))
        .route("/stats", get(crate::stats::handle))
        .with_state(state)
}
