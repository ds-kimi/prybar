//! Check, download and restart, each off the UI thread.

use std::sync::Arc;
use std::time::SystemTime;

use gpui::Context;
use prybar_update::{Status, Verified};

use super::Phase;
use crate::state::{AppState, ToastKind};

enum Step {
    Progress(Option<f32>),
    Done(Result<Verified, String>),
}

impl AppState {
    /// Asks for the latest release. A failed launch check stays quiet; a
    /// failed "Check now" says why.
    pub fn check_updates(&mut self, manual: bool, cx: &mut Context<Self>) {
        if matches!(self.updates.phase, Phase::Checking | Phase::Downloading(_)) {
            return;
        }
        self.updates.phase = Phase::Checking;
        cx.notify();
        let check = self.updates.backend.check;
        let task = cx.background_executor().spawn(async move { check() });
        cx.spawn(async move |this, cx| {
            let status = task.await;
            let _ = this.update(cx, |s, cx| s.on_checked(status, manual, cx));
        })
        .detach();
    }

    fn on_checked(&mut self, status: Status, manual: bool, cx: &mut Context<Self>) {
        self.updates.checked_at = Some(SystemTime::now());
        self.updates.phase = match status {
            Status::UpToDate => Phase::UpToDate,
            Status::Unknown(why) => {
                if manual {
                    self.toast(ToastKind::Error, format!("Update check: {why}"), cx);
                }
                Phase::Unknown(why)
            }
            Status::Available(r) => {
                self.updates.release = Some(r);
                Phase::Available
            }
        };
        if self.updates.phase == Phase::Available && self.updates.auto {
            self.start_update(cx);
        }
        cx.notify();
    }

    /// Downloads and verifies the found release; progress streams back
    /// so the button can show how far it is.
    pub fn start_update(&mut self, cx: &mut Context<Self>) {
        let Some(release) = self.updates.release.clone() else {
            return;
        };
        if matches!(self.updates.phase, Phase::Downloading(_) | Phase::Ready) {
            return;
        }
        self.updates.phase = Phase::Downloading(None);
        cx.notify();
        let (tx, rx) = async_channel::unbounded();
        let download = self.updates.backend.download;
        cx.background_executor()
            .spawn(async move {
                let mut report = |done: u64, total: Option<u64>| {
                    let part = total.filter(|t| *t > 0).map(|t| done as f32 / t as f32);
                    let _ = tx.try_send(Step::Progress(part));
                };
                let result = download(&release, &mut report);
                let _ = tx.try_send(Step::Done(result));
            })
            .detach();
        cx.spawn(async move |this, cx| {
            while let Ok(step) = rx.recv().await {
                let _ = this.update(cx, |s, cx| s.on_step(step, cx));
            }
        })
        .detach();
    }
}

impl AppState {
    fn on_step(&mut self, step: Step, cx: &mut Context<Self>) {
        match step {
            Step::Progress(part) => self.updates.phase = Phase::Downloading(part),
            Step::Done(Ok(v)) => {
                self.updates.staged = Some(Arc::new(v));
                self.updates.phase = Phase::Ready;
            }
            Step::Done(Err(e)) => {
                self.updates.phase = Phase::Failed(e.clone());
                self.toast(ToastKind::Error, format!("Update failed: {e}"), cx);
            }
        }
        cx.notify();
    }
}
