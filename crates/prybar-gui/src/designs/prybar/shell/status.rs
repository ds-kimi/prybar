//! Work in progress, shown inside the top bar so it never takes a row.

use gpui::{div, prelude::*, px, relative, AnyElement};

use super::super::motion;
use crate::state::{AppState, JobStatus};
use crate::theme::prybar::Tokens;

/// The running job's label, plus a count when several run at once.
/// Outcomes go to toasts, so the bar stays quiet when idle.
pub fn job(s: &AppState, t: Tokens) -> Option<AnyElement> {
    let mut running = s.jobs.iter().filter(|j| j.status == JobStatus::Running);
    let first = running.next()?;
    let text = match running.count() {
        0 => first.label.clone(),
        n => format!("{} and {n} more", first.label),
    };
    let el = div()
        .id("pb-job")
        .debug_selector(|| "pb-job".into())
        .px_2()
        .text_xs()
        .max_w_64()
        .overflow_hidden()
        .text_ellipsis()
        .whitespace_nowrap()
        .text_color(t.muted)
        .child(text);
    Some(el.into_any_element())
}

/// The bar's bottom edge carries the busy sweep while anything runs.
pub fn sweep(s: &AppState, t: Tokens) -> Option<AnyElement> {
    s.jobs
        .iter()
        .any(|j| j.status == JobStatus::Running)
        .then(|| {
            let band = div()
                .absolute()
                .top_0()
                .h_full()
                .w(relative(0.2))
                .bg(t.rust);
            div()
                .absolute()
                .left_0()
                .right_0()
                .bottom(px(-1.))
                .h(px(2.))
                .overflow_hidden()
                .child(motion::sweep(band))
                .into_any_element()
        })
}
