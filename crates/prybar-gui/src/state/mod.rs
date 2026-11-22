//! App state: what is open, where the user is,
//! and what runs in the background.

mod document;
mod job;
mod ops;
pub mod prefs;
mod run;
mod screen;
#[cfg(test)]
mod tests;

use std::path::PathBuf;

pub use document::Document;
pub use job::{Job, JobStatus, Toast, ToastKind};
pub use screen::{Placeholder, Screen};

use gpui::Context;
use prybar_service::model::DecompileReport;

/// Oldest toasts drop off past this, so errors never pile up forever.
const MAX_TOASTS: usize = 5;

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
    /// Index into the compile screen's game list.
    pub game: usize,
    /// Where the theme choice is saved; `None` keeps it in memory.
    pub prefs: Option<PathBuf>,
    next_id: u64,
}

impl AppState {
    pub fn navigate(&mut self, screen: Screen, cx: &mut Context<Self>) {
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

    pub fn toast(&mut self, kind: ToastKind, text: String, cx: &mut Context<Self>) {
        self.next_id += 1;
        self.toasts.push(Toast {
            id: self.next_id,
            kind,
            text,
        });
        if self.toasts.len() > MAX_TOASTS {
            self.toasts.remove(0);
        }
        cx.notify();
    }
}
