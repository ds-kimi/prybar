//! Design 3, Console: keyboard first and minimal. A command bar always
//! on top (fuzzy commands, file paths, recent files), monospace, high
//! contrast, almost no chrome; each screen is a pane that slides in, and
//! every row says which key reaches it. Motion is crisp: 80-120 ms.

mod commands;
mod motion;
pub mod run;
mod shell;
#[cfg(test)]
mod tests;
mod views;
mod widgets;

use gpui::{div, prelude::*, AnyElement, Context, Entity};

use crate::app::Root;
use crate::state::{AppState, DesignKind};
use crate::theme;

pub fn render(state: Entity<AppState>, cx: &mut Context<Root>) -> AnyElement {
    let s = state.read(cx);
    let p = theme::for_design(DesignKind::Console);
    div()
        .size_full()
        .relative()
        .flex()
        .flex_col()
        .bg(p.bg)
        .text_color(p.text)
        .font_family(theme::MONO)
        .text_sm()
        .child(shell::bar(s, p, cx))
        .child(shell::crumbs(s, p, cx))
        .child(views::screen(s, p, cx))
        .child(shell::foot::line(s, p))
        .children(s.search.is_some().then(|| shell::list::drop(s, p, cx)))
        .into_any_element()
}
