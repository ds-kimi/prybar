//! A compile in flight: its streamed log, the cancel flag, the result.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use gpui::Context;
use prybar_tools::compile::parse_log;

use crate::state::AppState;

/// The view keeps this many lines; a runaway compile drops the oldest.
const MAX_LINES: usize = 20_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Ordered plain, warning, error: views index colors by it.
pub enum LineKind {
    Plain,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub text: String,
    pub kind: LineKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileOutcome {
    Built {
        outputs: Vec<String>,
        warnings: usize,
    },
    Failed {
        errors: usize,
        exit: Option<i32>,
    },
    Cancelled,
    /// Could not run at all; the text says what to fix.
    Blocked(String),
}

#[derive(Default)]
pub struct CompileRun {
    pub lines: Vec<LogEntry>,
    pub running: bool,
    pub cancel: Arc<AtomicBool>,
    pub outcome: Option<CompileOutcome>,
}

impl AppState {
    /// Compiles `qc` with the picked game, streaming the log into the
    /// compile screen. With no game configured it says so and stops.
    pub fn compile(&mut self, qc: PathBuf, cx: &mut Context<Self>) {
        let Some(game) = self.tools.games.get(self.tools.game).cloned() else {
            let why = "No game is set up. Open Settings and detect games.";
            return self.compiled(Err(why.into()), cx);
        };
        let cancel = Arc::new(AtomicBool::new(false));
        self.tools.compile = CompileRun::started(cancel.clone());
        let (tx, rx) = async_channel::unbounded::<String>();
        let compile = self.tools.backend.compile;
        let work = cx.background_executor().spawn(async move {
            compile(
                &game,
                &qc,
                &mut |l| drop(tx.try_send(l.to_owned())),
                &cancel,
            )
        });
        cx.spawn(async move |this, cx| {
            while let Ok(line) = rx.recv().await {
                if this.update(cx, |s, cx| s.compile_line(line, cx)).is_err() {
                    return;
                }
            }
            let result = work.await;
            let _ = this.update(cx, |s, cx| s.compiled(result, cx));
        })
        .detach();
        cx.notify();
    }

    pub fn cancel_compile(&mut self) {
        self.tools.compile.cancel.store(true, Ordering::Relaxed);
    }

    fn compile_line(&mut self, text: String, cx: &mut Context<Self>) {
        let (errors, warnings) = parse_log(&text);
        let kind = match (errors.is_empty(), warnings.is_empty()) {
            (false, _) => LineKind::Error,
            (_, false) => LineKind::Warning,
            _ => LineKind::Plain,
        };
        let lines = &mut self.tools.compile.lines;
        if lines.len() >= MAX_LINES {
            lines.remove(0);
        }
        lines.push(LogEntry { text, kind });
        cx.notify();
    }
}
