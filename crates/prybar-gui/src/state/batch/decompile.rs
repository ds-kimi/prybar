use std::path::PathBuf;
use std::sync::atomic::Ordering;

use gpui::Context;
use prybar_service::model::{self, DecompileOptions, DecompileReport, CANCELLED};

use super::{BatchRun, ItemState, Msg};
use crate::state::{AppState, ToastKind};

impl AppState {
    /// Decompiles `arg` (a folder, a model or `package:`) with the
    /// screen's options, listing the models first so the screen shows
    /// them queued, then each one as it starts and ends.
    pub fn decompile_batch(&mut self, arg: String, out: PathBuf, cx: &mut Context<Self>) {
        let b = &mut self.batch;
        b.decompile = BatchRun::started(Some(out.clone()));
        let opts = DecompileOptions {
            out,
            recursive: !b.shallow,
            split_qc: self.split_qc,
            flat: b.flat,
            with_includes: !b.skip_includes,
            cancel: Some(b.decompile.cancel.clone()),
        };
        let work = move |tx: async_channel::Sender<Msg>| {
            if let Ok(found) = prybar_service::source::expand(&arg, opts.recursive) {
                let keys = found.stems.iter().map(|s| format!("{s}.mdl")).collect();
                drop(tx.try_send(Msg::Found(keys)));
            }
            let started = |m: &str| drop(tx.try_send(Msg::Started(m.to_owned())));
            model::decompile(&[arg], &opts, &started).map_err(|e| e.to_string())
        };
        self.stream_batch(work, |b| &mut b.decompile, decompiled, cx);
    }

    /// Models not started yet are skipped; the ones writing finish.
    pub fn cancel_batch(&mut self) {
        for run in [&self.batch.decompile, &self.batch.compile] {
            run.cancel.store(true, Ordering::Relaxed);
        }
    }
}

/// Marks every item from the report, keeps it for the summary and words
/// the toast. Missing includes and renamed folders are not failures but
/// the user still needs to know the output is not exactly what they asked.
fn decompiled(
    s: &mut AppState,
    result: Result<DecompileReport, String>,
    cx: &mut Context<AppState>,
) {
    let report = match result {
        Ok(r) => r,
        Err(e) => return s.toast(ToastKind::Error, e, cx),
    };
    settle(&mut s.batch.decompile, &report);
    let mut msg = format!(
        "Decompiled {} model(s), {} failed",
        report.models.len(),
        report.failures.len()
    );
    if !report.warnings.is_empty() {
        msg.push_str(&format!(", {} warning(s)", report.warnings.len()));
    }
    s.last_decompile = Some(report);
    s.toast(ToastKind::Info, msg, cx);
}

fn settle(run: &mut BatchRun, report: &DecompileReport) {
    for m in &report.models {
        run.mark(&m.input, ItemState::Done);
    }
    for f in &report.failures {
        let state = match f.error.as_str() {
            CANCELLED => ItemState::Cancelled,
            e => ItemState::Failed(e.to_owned()),
        };
        run.mark(&f.input, state);
    }
}
