//! The step path: drop -> inspect -> decompile -> done, with a fill that
//! springs forward as the file moves along.

use gpui::{div, prelude::*, px, AnyElement};

use super::super::motion;
use crate::state::{AppState, JobStatus};
use crate::theme::Palette;

const STEPS: [&str; 4] = ["Drop", "Inspect", "Decompile", "Done"];
/// Width of one step slot; the fill grows in steps of it.
const SLOT: f32 = 150.;

/// How far along the selected file is, 0 to 3.
fn stage(s: &AppState) -> usize {
    let running = s
        .jobs
        .iter()
        .any(|j| j.label == "Decompile" && j.status == JobStatus::Running);
    let done = s
        .last_decompile
        .as_ref()
        .is_some_and(|r| !r.models.is_empty());
    match (s.selected.is_some(), running, done) {
        (_, _, true) => 3,
        (_, true, _) => 2,
        (true, _, _) => 1,
        _ => 0,
    }
}

pub fn path(s: &AppState, p: Palette) -> AnyElement {
    let at = stage(s);
    let track = track(at, p);
    let dots = STEPS
        .iter()
        .enumerate()
        .map(|(ix, name)| step(name, ix, at, p));
    div()
        .id("cv-steps")
        .debug_selector(|| "cv-steps".into())
        .relative()
        .w(px(SLOT * 4.))
        .flex()
        .child(track)
        .children(dots)
        .into_any_element()
}

/// A numbered dot: filled once reached, ringed while it is the current one.
fn step(name: &'static str, ix: usize, at: usize, p: Palette) -> impl IntoElement {
    let reached = ix <= at;
    let mark = match reached && ix < at {
        true => "✓".to_string(),
        false => (ix + 1).to_string(),
    };
    let tone = if reached { p.text } else { p.muted };
    let dot = div()
        .size(px(30.))
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .border_2()
        .border_color(if reached { p.accent } else { p.border })
        .bg(if reached { p.accent } else { p.surface })
        .text_color(if reached { p.surface } else { p.muted })
        .text_sm()
        .when(ix == at, |el| el.shadow_md())
        .child(mark);
    div()
        .w(px(SLOT))
        .flex()
        .flex_col()
        .items_center()
        .gap_1()
        .child(dot)
        .child(div().text_xs().text_color(tone).child(name))
}

/// The line behind the dots; its fill springs one slot per step.
fn track(at: usize, p: Palette) -> impl IntoElement {
    let fill = div()
        .absolute()
        .top_0()
        .left_0()
        .h_full()
        .rounded_full()
        .bg(p.accent);
    div()
        .absolute()
        .top(px(13.))
        .left(px(SLOT / 2.))
        .w(px(SLOT * 3.))
        .h(px(4.))
        .rounded_full()
        .bg(p.active)
        .child(motion::spring_to(
            fill,
            "cv-path-fill",
            SLOT * at as f32,
            true,
        ))
}
