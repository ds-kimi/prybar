//! How a finished compile is worded on the compile screen.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use gpui::Context;
use prybar_tools::compile::{written_files, CompileReport};

use super::{CompileOutcome, CompileRun};
use crate::state::{AppState, ToastKind};

impl CompileRun {
    pub(super) fn started(cancel: Arc<AtomicBool>) -> Self {
        CompileRun {
            running: true,
            cancel,
            ..CompileRun::default()
        }
    }
}

impl AppState {
    pub(super) fn compiled(
        &mut self,
        result: Result<CompileReport, String>,
        cx: &mut Context<Self>,
    ) {
        let run = &mut self.tools.compile;
        run.running = false;
        let outcome = outcome(result, run.cancel.load(Ordering::Relaxed));
        let toast = match &outcome {
            CompileOutcome::Built { .. } => (ToastKind::Info, "Compiled".to_owned()),
            CompileOutcome::Cancelled => (ToastKind::Info, "Compile cancelled".to_owned()),
            CompileOutcome::Failed { errors, .. } => (
                ToastKind::Error,
                format!("Compile failed with {errors} error(s)"),
            ),
            CompileOutcome::Blocked(why) => (ToastKind::Error, why.clone()),
        };
        run.outcome = Some(outcome);
        self.toast(toast.0, toast.1, cx);
    }
}

/// A cancel wins over whatever the killed tool printed last.
fn outcome(result: Result<CompileReport, String>, cancelled: bool) -> CompileOutcome {
    match result {
        _ if cancelled => CompileOutcome::Cancelled,
        Err(why) => CompileOutcome::Blocked(why),
        Ok(r) if r.success => CompileOutcome::Built {
            outputs: written_files(&r.log),
            warnings: r.warnings.len(),
        },
        Ok(r) => CompileOutcome::Failed {
            errors: r.errors.len(),
            exit: r.exit_code,
        },
    }
}

impl CompileOutcome {
    /// The result in one line for the compile screen.
    pub fn describe(&self) -> String {
        match self {
            Self::Built { outputs, warnings } if outputs.is_empty() => {
                format!("Compiled, {warnings} warning(s)")
            }
            Self::Built { outputs, warnings } => {
                format!(
                    "Compiled, {warnings} warning(s). Wrote {}",
                    outputs.join(", ")
                )
            }
            Self::Failed { errors, exit } => match exit {
                Some(code) => format!("Failed: {errors} error(s), exit code {code}"),
                None => format!("Failed: {errors} error(s), studiomdl stopped"),
            },
            Self::Cancelled => "Cancelled".into(),
            Self::Blocked(why) => why.clone(),
        }
    }
}
