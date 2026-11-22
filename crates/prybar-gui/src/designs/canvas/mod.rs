//! Design 2, Canvas: friendly and visual, for modders who never used a
//! CLI. A top bar with big pill tabs, a step path that shows where the
//! file is in drop -> inspect -> decompile -> done, roomy cards, and
//! springy motion.

mod motion;
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
    let p = theme::for_design(DesignKind::Canvas);
    let main = div()
        .flex_1()
        .min_h_0()
        .flex()
        .flex_col()
        .px_8()
        .pb_6()
        .gap_4()
        .child(shell::steps::path(s, p))
        .child(shell::files(s, p, cx))
        .child(views::screen(s, p, cx));
    div()
        .size_full()
        .relative()
        .flex()
        .flex_col()
        .bg(p.bg)
        .text_color(p.text)
        .child(shell::top(s, p, cx))
        .child(main)
        .child(shell::toasts::stack(s, p))
        .into_any_element()
}
