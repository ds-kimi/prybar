//! Auto-update: check on launch, download and verify in the background,
//! then swap the app on "Restart to update".

pub mod backend;
mod flow;
#[cfg(test)]
mod tests;

use std::sync::Arc;
use std::time::SystemTime;

pub use backend::Backend;
use gpui::Context;
use prybar_update::{Release, Verified};

use crate::state::{AppState, ToastKind};

/// Version of this build, compared against the latest release.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Where the update flow stands; the nav button and Settings read it.
#[derive(Clone, Debug, PartialEq)]
pub enum Phase {
    Idle,
    Checking,
    UpToDate,
    /// The check failed (offline, rate limit); nothing is shown in the bar.
    Unknown(String),
    Available,
    /// Fraction downloaded, when the size is known.
    Downloading(Option<f32>),
    Ready,
    Failed(String),
}

pub struct Updates {
    pub phase: Phase,
    pub release: Option<Release>,
    /// The verified archive waiting for a restart; its temp dir lives here.
    pub staged: Option<Arc<Verified>>,
    pub checked_at: Option<SystemTime>,
    /// Download as soon as a release is found; on unless turned off.
    pub auto: bool,
    pub backend: Backend,
}

impl Default for Updates {
    fn default() -> Self {
        Self {
            phase: Phase::Idle,
            release: None,
            staged: None,
            checked_at: None,
            auto: true,
            backend: Backend::default(),
        }
    }
}

impl Updates {
    /// Where to send the user when installing in place is not possible.
    pub fn link(&self) -> String {
        self.release.as_ref().map_or_else(
            || "https://github.com/ds-kimi/prybar/releases/latest".into(),
            |r| r.page.clone(),
        )
    }
}

impl AppState {
    /// Saved for next launch; turning it on picks up a waiting release.
    pub fn set_auto_update(&mut self, on: bool, cx: &mut Context<Self>) {
        self.updates.auto = on;
        self.save_prefs(cx);
        if on && self.updates.phase == Phase::Available {
            self.start_update(cx);
        }
        cx.notify();
    }

    /// Installs the staged update and quits so the new copy takes over;
    /// on failure the app keeps running and says what went wrong.
    pub fn restart_to_update(&mut self, cx: &mut Context<Self>) {
        let Some(staged) = self.updates.staged.clone() else {
            return;
        };
        match (self.updates.backend.restart)(&staged) {
            Ok(()) => cx.quit(),
            Err(e) => {
                self.updates.phase = Phase::Failed(e.clone());
                self.toast(ToastKind::Error, e, cx);
            }
        }
    }
}
