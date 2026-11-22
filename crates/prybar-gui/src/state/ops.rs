//! The user-facing operations, each one a background job.

use std::path::PathBuf;

use gpui::Context;
use prybar_service::model::{DecompileOptions, DecompileReport};

use super::{AppState, Document, Screen};

/// Recent files kept for the open screen.
const MAX_RECENT: usize = 10;

impl AppState {
    /// Opens a model or package and shows it once it is read.
    pub fn open(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let label = format!("Open {}", path.display());
        let read = path.clone();
        self.run_job(
            label,
            move || Document::load(&read),
            move |state, doc| {
                let screen = match doc {
                    Document::Model { .. } => Screen::Inspector,
                    Document::Package { .. } => Screen::Package,
                };
                let title = doc.title();
                state.recent.retain(|p| p != &path);
                state.recent.insert(0, path);
                state.recent.truncate(MAX_RECENT);
                state.documents.push(doc);
                state.selected = Some(state.documents.len() - 1);
                if screen != state.screen {
                    state.history.push(state.screen);
                    state.screen = screen;
                }
                format!("Opened {title}")
            },
            cx,
        );
    }

    /// Decompiles the selected document (a model, or every model in a package).
    pub fn decompile_selected(&mut self, out: PathBuf, cx: &mut Context<Self>) {
        let Some(doc) = self.selected.and_then(|i| self.documents.get(i)) else {
            return;
        };
        let mut arg = doc.path().to_string_lossy().into_owned();
        if matches!(doc, Document::Package { .. }) {
            // An empty inner path names every model in the package.
            arg.push(':');
        }
        let opts = DecompileOptions {
            out,
            recursive: true,
            split_qc: self.split_qc,
        };
        let work = move || {
            prybar_service::model::decompile(&[arg], &opts, &|_| {}).map_err(|e| e.to_string())
        };
        self.run_job("Decompile".into(), work, decompiled, cx);
    }

    /// Extracts every file of the selected package into `out`.
    pub fn extract_selected(&mut self, out: PathBuf, cx: &mut Context<Self>) {
        let Some(Document::Package { path, .. }) =
            self.selected.and_then(|i| self.documents.get(i))
        else {
            return;
        };
        let path = path.clone();
        let work = move || prybar_service::ops::unpack(&path, &out).map_err(|e| e.to_string());
        self.run_job(
            "Extract".into(),
            work,
            |_, report| {
                format!(
                    "Extracted {} file(s) to {}",
                    report.files,
                    report.out.display()
                )
            },
            cx,
        );
    }
}

/// Keeps the report for the decompile screen and words the toast.
fn decompiled(state: &mut AppState, report: DecompileReport) -> String {
    let msg = format!(
        "Decompiled {} model(s), {} failed",
        report.models.len(),
        report.failures.len()
    );
    state.last_decompile = Some(report);
    msg
}
