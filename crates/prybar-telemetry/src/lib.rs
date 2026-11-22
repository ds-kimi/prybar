//! Anonymous analytics for the GUI: two event kinds, a local outbox, a
//! log of everything queued or sent, and the kill switches.
//!
//! No install, user or hardware ID exists anywhere in here, so the
//! collector can only count events, never people.

mod env;
mod event;
pub mod report;
mod send;
mod store;

pub use env::{blocked, blocked_by};
pub use event::{Event, MAX_FRAMES, MAX_MESSAGE};
pub use send::{flush, http, Transport};
pub use store::{Entry, Store, MAX_QUEUED};

/// Collector URL baked in at build time (`PRYBAR_TELEMETRY_URL`). Empty
/// means analytics are "not configured": the UI works, nothing is sent.
pub const ENDPOINT: &str = match option_env!("PRYBAR_TELEMETRY_URL") {
    Some(url) => url,
    None => "",
};
