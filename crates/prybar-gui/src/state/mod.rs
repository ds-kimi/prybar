//! App state: what is open, where the user is,
//! and what runs in the background.

pub mod batch;
pub mod checks;
pub(crate) mod companions;
mod document;
mod job;
mod ops;
pub mod prefs;
mod run;
mod screen;
#[cfg(test)]
pub(crate) mod tests;
pub mod toast;
pub mod tools;

use std::path::PathBuf;

pub use document::Document;
pub use job::{Job, JobStatus, Toast, ToastKind};
pub use screen::Screen;
pub use tools::Tools;

use gpui::Context;
use prybar_service::model::DecompileReport;

#[derive(Default)]
pub struct AppState {
    pub screen: Screen,
    pub history: Vec<Screen>,
    pub documents: Vec<Document>,
    pub selected: Option<usize>,
    pub recent: Vec<PathBuf>,
    pub jobs: Vec<Job>,
    pub toasts: Vec<Toast>,
    /// `Some` while the search field is open.
    pub search: Option<String>,
    pub last_decompile: Option<DecompileReport>,
    /// The command palette is open.
    pub palette: bool,
    /// Light theme instead of the dark default.
    pub light: bool,
    /// Decompile option: one `.qci` per QC section.
    pub split_qc: bool,
    /// Folder decompile and compile: options, items, progress.
    pub batch: batch::Batches,
    /// Games config, compile and publish runs.
    pub tools: Tools,
    /// Release check, download and restart.
    pub updates: crate::update::Updates,
    /// Analytics choice, outbox and the log of what was sent.
    pub privacy: crate::telemetry::Privacy,
    /// Where the theme choice is saved; `None` keeps it in memory.
    pub prefs: Option<PathBuf>,
    next_id: u64,
}

impl AppState {
    pub fn navigate(&mut self, screen: Screen, cx: &mut Context<Self>) {
        if matches!(screen, Screen::Compile | Screen::Settings) {
            self.ensure_games(cx);
        }
        if screen == Screen::Publish && !self.tools.steam_started {
            self.refresh_steam(cx);
        }
        if screen != self.screen {
            self.history.push(self.screen);
            self.screen = screen;
            cx.notify();
        }
    }

    /// Back closes the palette or search first, then walks screen
    /// history; on the first screen it does nothing.
    pub fn back(&mut self, cx: &mut Context<Self>) {
        if self.palette || self.search.is_some() {
            self.palette = false;
            self.search = None;
            cx.notify();
            return;
        }
        if let Some(screen) = self.history.pop() {
            self.screen = screen;
            cx.notify();
        }
    }
}
