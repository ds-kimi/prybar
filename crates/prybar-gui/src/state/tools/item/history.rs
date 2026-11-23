//! The selected item's changelog, asked politely: only for the item
//! that stays selected, one request at a time, and after a busy answer
//! only once the wait Steam asked for (or a growing one) has passed.

use std::time::Duration;

use gpui::Context;
use prybar_workshop::changelog::backoff;
use prybar_workshop::http::Failure;

use super::History;
use crate::state::AppState;

impl AppState {
    /// Asks for `id`'s history after `delay`, if it is still selected
    /// and still wants it then. A loaded or failed history stays as is.
    pub(super) fn plan_history(&mut self, id: u64, delay: Duration, cx: &mut Context<Self>) {
        let h = &mut self.tools.item.history;
        match h.get(&id) {
            None | Some(History::Failed(_)) => drop(h.insert(id, History::Waiting(0))),
            Some(History::Waiting(_) | History::Busy { .. }) => {}
            Some(History::Loading(_) | History::Ready(_)) => return,
        }
        let timer = cx.background_executor().timer(delay);
        cx.spawn(async move |this, cx| {
            timer.await;
            let _ = this.update(cx, |s, cx| s.fetch_history(id, cx));
        })
        .detach();
    }

    /// The Try again button: asks right away, the wait is skipped.
    pub fn retry_history(&mut self, id: u64, cx: &mut Context<Self>) {
        let attempt = match self.tools.item.history.get(&id) {
            Some(History::Busy { attempt, .. }) => *attempt,
            Some(History::Failed(_)) => 0,
            _ => return,
        };
        self.tools.item.history.insert(id, History::Waiting(attempt));
        self.fetch_history(id, cx);
        cx.notify();
    }

    fn fetch_history(&mut self, id: u64, cx: &mut Context<Self>) {
        let v = &mut self.tools.item;
        let attempt = match v.history.get(&id) {
            Some(History::Waiting(a)) => *a,
            Some(History::Busy { attempt, .. }) => *attempt,
            _ => return,
        };
        // Another request is out: this one waits for it (see `got`).
        if v.selected != Some(id) || v.fetching {
            return;
        }
        v.fetching = true;
        v.history.insert(id, History::Loading(attempt));
        let fetch = self.tools.backend.history;
        let work = cx.background_executor().spawn(async move { fetch(id) });
        cx.spawn(async move |this, cx| {
            let got = work.await;
            let _ = this.update(cx, |s, cx| s.got_history(id, attempt, got, cx));
        })
        .detach();
        cx.notify();
    }

    fn got_history(
        &mut self,
        id: u64,
        attempt: u32,
        got: Result<prybar_workshop::changelog::Log, Failure>,
        cx: &mut Context<Self>,
    ) {
        let mut wait = None;
        let next = match got {
            Ok(log) => History::Ready(log),
            Err(e) if e.busy() => {
                let w = backoff(attempt, e.retry_after);
                wait = Some(w);
                History::Busy { wait: w.as_secs(), attempt: attempt + 1 }
            }
            Err(e) => History::Failed(e.message),
        };
        let v = &mut self.tools.item;
        v.fetching = false;
        v.history.insert(id, next);
        if let Some(w) = wait {
            self.plan_history(id, w, cx);
        }
        let v = &self.tools.item;
        // The selection moved while this one was out: its turn now.
        if let Some(other) = v.selected.filter(|&s| s != id) {
            self.fetch_history(other, cx);
        }
        cx.notify();
    }
}
