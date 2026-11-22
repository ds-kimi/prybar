//! Toasts float bottom right as rounded bubbles that pop in.

use gpui::{div, prelude::*, AnyElement};

use super::super::motion;
use gpui::Div;

use crate::state::{AppState, Toast, ToastKind};
use crate::theme::Palette;

/// Newest at the bottom; errors get the danger edge.
pub fn stack(s: &AppState, p: Palette) -> AnyElement {
    let bubbles = s
        .toasts
        .iter()
        .map(|t| motion::enter(bubble(t, p), ("cv-toast", t.id as usize), 0));
    div()
        .id("cv-toasts")
        .debug_selector(|| "cv-toasts".into())
        .absolute()
        .bottom_4()
        .right_4()
        .flex()
        .flex_col()
        .items_end()
        .gap_2()
        .children(bubbles)
        .into_any_element()
}

fn bubble(t: &Toast, p: Palette) -> Div {
    let edge = match t.kind {
        ToastKind::Error => p.danger,
        ToastKind::Info => p.accent,
    };
    div()
        .max_w_96()
        .px_4()
        .py_2()
        .rounded_2xl()
        .border_l_4()
        .border_color(edge)
        .bg(p.surface)
        .shadow_lg()
        .text_sm()
        .child(t.text.clone())
}
