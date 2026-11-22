//! Launch, the notice answer, the switch and the log buttons.

use gpui::Context;

use crate::state::{AppState, ToastKind};
use crate::update::VERSION;

impl AppState {
    /// On launch: drop what waits when off, otherwise queue `app_start`
    /// and send the outbox (crash reports from last time included) in the
    /// background. Before the notice is answered nothing is queued.
    pub fn start_telemetry(&mut self, cx: &mut Context<Self>) {
        let p = &self.privacy;
        let Some(store) = p.store.clone() else {
            return;
        };
        if !p.on {
            let _ = store.drop_queue();
        } else if p.sending() {
            let _ = store.queue(&prybar_telemetry::Event::app_start(VERSION));
            let (endpoint, transport) = (p.endpoint, p.transport);
            let task = cx.background_executor().spawn(async move {
                prybar_telemetry::flush(&store, endpoint, transport);
                store.entries()
            });
            cx.spawn(async move |this, cx| {
                let log = task.await;
                let _ = this.update(cx, |s, cx| {
                    s.privacy.log = log;
                    cx.notify();
                });
            })
            .detach();
        }
        self.reload_sent_log();
        cx.notify();
    }

    /// "Keep on" and "Turn off" both end the notice; keeping on starts
    /// sending right away, turning off deletes anything queued.
    pub fn answer_notice(&mut self, keep: bool, cx: &mut Context<Self>) {
        self.privacy.notice_seen = true;
        self.set_telemetry(keep, cx);
        if keep {
            self.start_telemetry(cx);
        }
    }

    /// Off stops sending at once and deletes queued reports.
    pub fn set_telemetry(&mut self, on: bool, cx: &mut Context<Self>) {
        self.privacy.on = on;
        if !on {
            if let Some(store) = &self.privacy.store {
                if let Err(e) = store.drop_queue() {
                    self.toast(
                        ToastKind::Error,
                        format!("Queued reports not deleted: {e}"),
                        cx,
                    );
                }
            }
        }
        self.save_prefs(cx);
        self.reload_sent_log();
        cx.notify();
    }

    pub fn clear_sent_log(&mut self, cx: &mut Context<Self>) {
        if let Some(Err(e)) = self.privacy.store.as_ref().map(|s| s.clear_log()) {
            self.toast(ToastKind::Error, format!("Log not cleared: {e}"), cx);
        }
        self.privacy.open = None;
        self.reload_sent_log();
        cx.notify();
    }
}
