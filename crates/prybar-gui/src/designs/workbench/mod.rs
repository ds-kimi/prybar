//! Design 1, Workbench: a dense, calm pro tool. Sidebar on the left, the
//! current screen in the middle under file tabs, the inspector on the
//! right, a status bar with job progress, and a command palette.

mod motion;
mod shell;
#[cfg(test)]
mod tests;
mod views;
mod widgets;

use gpui::{div, prelude::*, AnyElement, Context, Entity};

use crate::app::Root;
use crate::state::AppState;
use crate::theme;

pub fn render(state: Entity<AppState>, cx: &mut Context<Root>) -> AnyElement {
    let s = state.read(cx);
    let p = theme::workbench(s.light);
    let center = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .child(shell::tabs::bar(s, p, cx))
        .child(views::screen(s, p, cx));
    let body = div()
        .flex_1()
        .min_h_0()
        .flex()
        .child(shell::sidebar(s, p, cx))
        .child(center)
        .child(shell::inspector::panel(s, p));
    div()
        .size_full()
        .relative()
        .flex()
        .flex_col()
        .text_sm()
        .child(body)
        .child(shell::status::bar(s, p))
        .children(s.palette.then(|| shell::palette::overlay(p, cx)))
        .into_any_element()
}
