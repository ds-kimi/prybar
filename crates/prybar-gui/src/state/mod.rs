//! App state shared by every design: what is open, where the user is,
//! and what runs in the background.

mod design;
mod document;
mod job;
mod ops;
mod run;
mod screen;
#[cfg(test)]
mod tests;

use std::path::PathBuf;

pub use design::DesignKind;
pub use document::Document;
pub use job::{Job, JobStatus, Toast, ToastKind};
pub use screen::{Placeholder, Screen};

use gpui::Context;
use prybar_service::model::DecompileReport;

/// Oldest toasts drop off past this, so errors never pile up forever.
const MAX_TOASTS: usize = 5;

#[derive(Default)]
pub struct AppState {
    /// `None` shows the chooser.
    pub design: Option<DesignKind>,
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
    next_id: u64,
}

impl AppState {
    pub fn set_design(&mut self, design: Option<DesignKind>, cx: &mut Context<Self>) {
        self.design = design;
        cx.notify();
    }

    pub fn navigate(&mut self, screen: Screen, cx: &mut Context<Self>) {
        if screen != self.screen {
            self.history.push(self.screen);
            self.screen = screen;
            cx.notify();
        }
    }

    /// Back walks screen history first, then leaves the design for the chooser.
    pub fn back(&mut self, cx: &mut Context<Self>) {
        match self.history.pop() {
            Some(screen) => self.screen = screen,
            None => self.design = None,
        }
        cx.notify();
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
