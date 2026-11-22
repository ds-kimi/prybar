//! Runs prybar-service calls off the UI thread and reports back.

use gpui::Context;

use super::{AppState, Job, JobStatus, ToastKind};

impl AppState {
    pub fn select(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.documents.len() {
            self.selected = Some(index);
            cx.notify();
        }
    }

    /// Starts `work` on the background executor and tracks it as a job.
    /// `done` applies the result on the UI thread; its return value is
    /// the success message. Errors become a failed job and an error toast.
    pub fn run_job<R: Send + 'static>(
        &mut self,
        label: String,
        work: impl FnOnce() -> Result<R, String> + Send + 'static,
        done: impl FnOnce(&mut AppState, R) -> String + 'static,
        cx: &mut Context<Self>,
    ) {
        self.next_id += 1;
        let id = self.next_id;
        self.jobs.push(Job {
            id,
            label,
            status: JobStatus::Running,
        });
        cx.notify();
        let task = cx.background_executor().spawn(async move { work() });
        cx.spawn(async move |this, cx| {
            let result = task.await;
            // The window may be gone by now; nothing left to update then.
            let _ = this.update(cx, |state, cx| {
                let status = match result {
                    Ok(value) => JobStatus::Done(done(state, value)),
                    Err(e) => JobStatus::Failed(e),
                };
                state.finish_job(id, status, cx);
            });
        })
        .detach();
    }

    fn finish_job(&mut self, id: u64, status: JobStatus, cx: &mut Context<Self>) {
        let (kind, text) = match &status {
            JobStatus::Failed(e) => (ToastKind::Error, e.clone()),
            JobStatus::Done(msg) => (ToastKind::Info, msg.clone()),
            JobStatus::Running => return,
        };
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == id) {
            job.status = status;
        }
        self.toast(kind, text, cx);
    }
}
