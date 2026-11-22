//! The app's design: a calm hierarchy under a horizontal top bar, dark
//! by default with a designed light theme, rust as the only accent.

mod motion;
mod shell;
#[cfg(test)]
mod tests;
mod views;
mod widgets;

use gpui::{div, prelude::*, AnyElement, Context, Entity};

use crate::app::Root;
use crate::state::AppState;
use crate::theme::prybar::tokens;

pub fn render(state: Entity<AppState>, cx: &mut Context<Root>) -> AnyElement {
    let s = state.read(cx);
    let t = tokens(s.light);
    div()
        .size_full()
        .relative()
        .flex()
        .flex_col()
        .bg(t.bg)
        .text_color(t.text)
        .text_sm()
        .child(shell::bar(s, t, cx))
        .child(views::screen(s, t, cx))
        .child(shell::toasts::stack(s, t))
        .children(s.palette.then(|| shell::palette::overlay(t, cx)))
        .into_any_element()
}
