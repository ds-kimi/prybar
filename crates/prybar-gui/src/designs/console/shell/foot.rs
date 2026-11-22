//! One line at the bottom: key hints, the running job, and the last
//! message, which slides in when it changes.

use gpui::{div, prelude::*, AnyElement, Div};

use super::super::{motion, widgets};
use crate::actions::MOD;
use crate::state::{AppState, JobStatus, ToastKind};
use crate::theme::Palette;

pub fn line(s: &AppState, p: Palette) -> AnyElement {
    div()
        .id("cs-foot")
        .debug_selector(|| "cs-foot".into())
        .h_7()
        .px_3()
        .flex()
        .items_center()
        .gap_4()
        .border_t_1()
        .border_color(p.border)
        .text_xs()
        .children(hints(p))
        .child(div().flex_1())
        .child(status(s, p))
        .into_any_element()
}

/// The keys that matter most, always on screen.
fn hints(p: Palette) -> impl Iterator<Item = Div> {
    let hints = [
        ("⏎", "run"),
        ("↑↓", "pick"),
        ("esc", "back"),
        ("o", "open"),
        ("3", "console"),
    ];
    hints.into_iter().map(move |(k, what)| {
        // Single letters are shortcuts with the command key.
        let k = if k.len() == 1 {
            format!("{MOD}-{k}")
        } else {
            k.into()
        };
        div()
            .flex()
            .gap_1()
            .child(widgets::key(k, p))
            .child(div().text_color(p.muted).child(what))
    })
}

/// A running job wins over the last message; errors read in red.
fn status(s: &AppState, p: Palette) -> AnyElement {
    if let Some(job) = s.jobs.iter().rev().find(|j| j.status == JobStatus::Running) {
        let text = format!("… {}", job.label);
        return motion::caret(div().text_color(p.accent).child(text));
    }
    let Some(t) = s.toasts.last() else {
        return div().text_color(p.muted).child("ready").into_any_element();
    };
    let color = match t.kind {
        ToastKind::Error => p.danger,
        ToastKind::Info => p.text,
    };
    let el = div().text_color(color).child(t.text.clone());
    motion::pane(el, ("cs-toast", t.id as usize))
}
