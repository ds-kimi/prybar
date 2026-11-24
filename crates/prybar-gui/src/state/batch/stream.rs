use async_channel::Sender;
use gpui::Context;

use super::{BatchRun, Batches, ItemState, Msg};
use crate::state::AppState;

impl AppState {
    /// Runs `work` off the UI thread, applying its messages to the run
    /// `pick` names as they come, then `done` with its result. Both
    /// happen in one task so the result always lands after the last
    /// message.
    pub(super) fn stream_batch<R: Send + 'static>(
        &mut self,
        work: impl FnOnce(Sender<Msg>) -> R + Send + 'static,
        pick: fn(&mut Batches) -> &mut BatchRun,
        done: impl FnOnce(&mut AppState, R, &mut Context<Self>) + 'static,
        cx: &mut Context<Self>,
    ) {
        let (tx, rx) = async_channel::unbounded::<Msg>();
        let task = cx.background_executor().spawn(async move { work(tx) });
        cx.spawn(async move |this, cx| {
            while let Ok(msg) = rx.recv().await {
                let applied = this.update(cx, |s, cx| {
                    apply(pick(&mut s.batch), msg);
                    cx.notify();
                });
                if applied.is_err() {
                    return;
                }
            }
            let result = task.await;
            let _ = this.update(cx, |s, cx| {
                pick(&mut s.batch).running = false;
                done(s, result, cx);
                cx.notify();
            });
        })
        .detach();
        cx.notify();
    }
}

fn apply(run: &mut BatchRun, msg: Msg) {
    match msg {
        Msg::Found(keys) => run.list(keys),
        Msg::Started(key) => run.mark(&key, ItemState::Running),
    }
}
