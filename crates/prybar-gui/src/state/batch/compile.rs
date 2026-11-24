use std::path::PathBuf;

use gpui::Context;
use prybar_service::tools::{qc_files, CompileBatchReport, CompileOptions, ItemStatus};

use super::{BatchRun, ItemState, Msg};
use crate::state::{AppState, ToastKind};

impl AppState {
    /// A folder compiles as a batch; a single QC streams its log, so
    /// the item list of an older batch goes away.
    pub fn compile_input(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        if path.is_dir() {
            return self.compile_folder(path, cx);
        }
        self.batch.compile = BatchRun::default();
        self.compile(path, cx);
    }

    /// Compiles every QC in `folder` with the picked game, a few at a
    /// time, listing them first so the screen shows them queued.
    pub fn compile_folder(&mut self, folder: PathBuf, cx: &mut Context<Self>) {
        let Some(game) = self.tools.games.get(self.tools.game).cloned() else {
            let why = "No game is set up. Open Settings and detect games.";
            return self.toast(ToastKind::Error, why.into(), cx);
        };
        self.batch.compile = BatchRun::started(None);
        let cancel = self.batch.compile.cancel.clone();
        let opts = CompileOptions {
            recursive: !self.batch.compile_shallow,
            jobs: None,
        };
        let compile = self.tools.backend.compile_batch;
        let work = move |tx: async_channel::Sender<Msg>| {
            let args = [folder.display().to_string()];
            let keys = qc_files(&args, opts.recursive).0;
            let keys = keys.iter().map(|q| q.display().to_string()).collect();
            drop(tx.try_send(Msg::Found(keys)));
            let started = |q: &str| drop(tx.try_send(Msg::Started(q.to_owned())));
            compile(&game, &args, &opts, &started, &cancel)
        };
        self.stream_batch(work, |b| &mut b.compile, compiled, cx);
    }
}

fn compiled(
    s: &mut AppState,
    result: Result<CompileBatchReport, String>,
    cx: &mut Context<AppState>,
) {
    let report = match result {
        Ok(r) => r,
        Err(e) => return s.toast(ToastKind::Error, e, cx),
    };
    let run = &mut s.batch.compile;
    for item in &report.items {
        run.mark(&item.qc, state(item));
        if run.out.is_none() {
            let dir = item.outputs.first().map(PathBuf::from);
            run.out = dir.and_then(|d| d.parent().map(PathBuf::from));
        }
    }
    let msg = format!(
        "Compiled {} QC(s), {} failed, {} cancelled",
        report.compiled, report.failed, report.cancelled
    );
    s.toast(ToastKind::Info, msg, cx);
}

/// The first error line says more than the exit code, so it is the
/// reason shown on hover.
fn state(item: &prybar_service::tools::CompileItem) -> ItemState {
    match item.status {
        ItemStatus::Ok => ItemState::Done,
        ItemStatus::Cancelled => ItemState::Cancelled,
        ItemStatus::Failed => {
            let first = item.errors.first().map(|l| l.text.clone());
            let why = first.or_else(|| item.error.clone());
            ItemState::Failed(why.unwrap_or_else(|| "studiomdl failed".into()))
        }
    }
}
