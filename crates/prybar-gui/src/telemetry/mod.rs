//! Analytics as the user sees them: on by default, nothing sent before
//! the first-launch notice was answered, every payload kept in a local
//! log the user can read, and off for good with one click.

mod flow;
mod launch;
#[cfg(test)]
mod tests;

use std::path::PathBuf;

pub use launch::launch;
use prybar_telemetry::{Entry, Store, Transport};

/// Public privacy page linked from the notice and Settings.
pub const PRIVACY_URL: &str = "https://github.com/ds-kimi/prybar/blob/main/PRIVACY.md";

pub struct Privacy {
    /// The user's choice; on unless turned off.
    pub on: bool,
    /// The first-launch notice was answered.
    pub notice_seen: bool,
    /// Outbox and log; `None` keeps tests away from the real config dir.
    pub store: Option<Store>,
    /// Collector URL; empty = "not configured", nothing is ever sent.
    pub endpoint: &'static str,
    /// An environment kill switch that wins over `on`.
    pub blocked: Option<&'static str>,
    /// The local log, newest last, as shown in Settings › Privacy.
    pub log: Vec<Entry>,
    /// Log row whose full JSON is expanded.
    pub open: Option<usize>,
    pub transport: Transport,
}

/// Tests start with the notice answered so it only shows where a test
/// asks for it; a real launch reads the saved answer instead.
impl Default for Privacy {
    fn default() -> Self {
        Self {
            on: true,
            notice_seen: true,
            store: None,
            endpoint: "",
            blocked: None,
            log: Vec::new(),
            open: None,
            transport: prybar_telemetry::http,
        }
    }
}

impl Privacy {
    /// Whether anything may leave the machine right now.
    pub fn sending(&self) -> bool {
        self.on
            && self.notice_seen
            && self.blocked.is_none()
            && !self.endpoint.is_empty()
            && self.store.is_some()
    }

    /// One line on why sending is or is not happening.
    pub fn status(&self) -> String {
        match (self.blocked, self.endpoint.is_empty(), self.on) {
            (Some(why), _, _) => format!("Off: {why}"),
            (None, true, _) => "Not configured in this build: nothing is sent".into(),
            (None, false, false) => "Off: nothing is sent".into(),
            (None, false, true) => format!("On: sent to {}", self.endpoint),
        }
    }
}

/// `telemetry/` next to the GUI prefs, so `PRYBAR_CONFIG` moves it too.
pub fn default_dir() -> Option<PathBuf> {
    Some(
        crate::state::prefs::default_path()?
            .parent()?
            .join("telemetry"),
    )
}
