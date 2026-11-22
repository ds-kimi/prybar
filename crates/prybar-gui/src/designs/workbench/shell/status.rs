//! Bottom bar: running jobs with a shimmering progress bar, the last
//! message, and the key hints that matter most.

use gpui::{div, prelude::*, px, AnyElement};

use super::super::motion;
use crate::actions::MOD;
use crate::state::{AppState, JobStatus, ToastKind};
use crate::theme::Palette;

/// Width of the progress track.
const TRACK: f32 = 120.;

pub fn bar(s: &AppState, p: Palette) -> AnyElement {
    let running: Vec<&str> = s
        .jobs
        .iter()
        .filter(|j| j.status == JobStatus::Running)
        .map(|j| j.label.as_str())
        .collect();
    let progress = (!running.is_empty()).then(|| track(running.join(", "), p));
    div()
        .id("wb-status")
        .debug_selector(|| "wb-status".into())
        .h_7()
        .flex_none()
        .px_3()
        .flex()
        .items_center()
        .gap_4()
        .text_xs()
        .bg(p.surface)
        .border_t_1()
        .border_color(p.border)
        .text_color(p.muted)
        .children(progress)
        .child(last_message(s, p))
        .child(div().flex_1())
        .child(format!(
            "{MOD}-k commands · {MOD}-o open · {MOD}-shift-l theme"
        ))
        .into_any_element()
}

fn track(label: String, p: Palette) -> AnyElement {
    let band = div()
        .absolute()
        .top_0()
        .bottom_0()
        .w(px(40.))
        .bg(p.accent)
        .opacity(0.85);
    let bar = div()
        .relative()
        .w(px(TRACK))
        .h(px(3.))
        .rounded_full()
        .overflow_hidden()
        .bg(p.active)
        .child(motion::shimmer(band, TRACK));
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(bar)
        .child(div().text_color(p.text).child(label))
        .into_any_element()
}

/// The newest toast doubles as the status line; errors stand out.
fn last_message(s: &AppState, p: Palette) -> AnyElement {
    let Some(t) = s.toasts.last() else {
        return div().child("Ready").into_any_element();
    };
    let color = match t.kind {
        ToastKind::Error => p.danger,
        ToastKind::Info => p.muted,
    };
    let text = div()
        .id(("wb-msg", t.id as usize))
        .text_color(color)
        .child(t.text.clone());
    motion::enter(div().child(text), ("wb-msg-in", t.id as usize))
}
