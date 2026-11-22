//! Toasts float bottom right. Errors say so in words and color, with no
//! stripe or icon badge.

use gpui::{div, prelude::*, AnyElement, Div, FontWeight};

use super::super::motion;
use crate::state::{AppState, Toast, ToastKind};
use crate::theme::prybar::Tokens;

/// Newest at the bottom, nearest the pointer's usual resting place.
pub fn stack(s: &AppState, t: Tokens) -> AnyElement {
    let items = s
        .toasts
        .iter()
        .map(|toast| motion::enter(card(toast, t), ("pb-toast", toast.id as usize)));
    div()
        .id("pb-toasts")
        .debug_selector(|| "pb-toasts".into())
        .absolute()
        .bottom_4()
        .right_4()
        .flex()
        .flex_col()
        .items_end()
        .gap_2()
        .children(items)
        .into_any_element()
}

fn card(toast: &Toast, t: Tokens) -> Div {
    let head = match toast.kind {
        ToastKind::Error => Some(
            div()
                .text_color(t.danger)
                .font_weight(FontWeight::MEDIUM)
                .child("Error"),
        ),
        ToastKind::Info => None,
    };
    div()
        .max_w_96()
        .px_3()
        .py_2()
        .flex()
        .gap_2()
        .rounded_lg()
        .border_1()
        .border_color(t.border)
        .bg(t.raised)
        .shadow_md()
        .children(head)
        .child(toast.text.clone())
}
