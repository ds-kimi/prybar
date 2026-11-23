//! Toasts float bottom right, newest at the bottom. Each slides in from
//! the right and back out; the room it takes opens and closes smoothly,
//! so the others glide instead of jumping. Errors say so in words and
//! color, with no stripe or icon badge. GPUI's animations already honor
//! the system's reduce-motion setting.

use gpui::{div, prelude::*, AnyElement, Context, Div, FontWeight, Stateful};

use super::super::motion;
use super::super::widgets::icon_button;
use super::super::widgets::Tipped as _;
use crate::app::Root;
use crate::state::{AppState, Toast, ToastKind};
use crate::theme::prybar::Tokens;

pub fn stack(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    div()
        .id("pb-toasts")
        .debug_selector(|| "pb-toasts".into())
        .absolute()
        .bottom_4()
        .right_4()
        .flex()
        .flex_col()
        .items_end()
        .children(s.toasts.iter().map(|toast| item(toast, t, cx)))
        .into_any_element()
}

/// The gap above a toast is part of its room, so it closes with it.
fn item(toast: &Toast, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let id = toast.id;
    let room = div()
        .debug_selector(move || format!("pb-toast-{id}"))
        .flex()
        .flex_col()
        .items_end()
        .overflow_hidden()
        .child(div().h_2())
        .child(card(toast, t, cx));
    motion::toast(room, toast.id as usize, toast.leaving)
}

/// Hover holds it, a click anywhere on it dismisses it.
fn card(toast: &Toast, t: Tokens, cx: &Context<Root>) -> Stateful<Div> {
    let id = toast.id;
    let head = div().text_color(t.danger).font_weight(FontWeight::MEDIUM);
    div()
        .id(("pb-toast", id as usize))
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
        .cursor_pointer()
        .on_hover(cx.listener(move |this, on: &bool, _, cx| {
            this.state.update(cx, |s, cx| s.hover_toast(id, *on, cx))
        }))
        .on_click(
            cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| s.dismiss_toast(id, cx))
            }),
        )
        .when(toast.kind == ToastKind::Error, |el| {
            el.child(head.child("Error"))
        })
        .child(div().flex_1().min_w_0().child(toast.text.clone()))
        .child(close(id, t))
}

/// A visible way out; the click itself lands on the card.
fn close(id: u64, t: Tokens) -> impl IntoElement {
    icon_button(("pb-toast-close", id as usize), "✕", t)
        .size_6()
        .tip("Dismiss", t)
}
