//! Small Workbench building blocks. Every interactive one reacts to
//! hover, press and keyboard focus, and shows a muted disabled state.

mod button;
mod field;

pub use button::button;
pub use field::{field, toggle};

use gpui::{div, prelude::*, Div, ElementId, Stateful};

use crate::theme::Palette;

/// Keyboard focus ring shared by every focusable widget.
pub fn ring(el: Stateful<Div>, p: Palette) -> Stateful<Div> {
    el.tab_index(0)
        .focus_visible(move |s| s.border_color(p.accent))
}

/// A list row: tinted on hover, darker while pressed, accented when selected.
pub fn row(id: impl Into<ElementId>, selected: bool, p: Palette) -> Stateful<Div> {
    div()
        .id(id)
        .h_7()
        .px_2()
        .flex()
        .items_center()
        .gap_2()
        .rounded_sm()
        .border_1()
        .border_color(gpui::transparent_black())
        .cursor_pointer()
        .text_color(if selected { p.text } else { p.muted })
        .hover(move |s| s.bg(p.hover).text_color(p.text))
        .active(move |s| s.bg(p.active))
}

/// A small caps heading above a group.
pub fn section(title: &'static str, p: Palette) -> Div {
    div()
        .pt_3()
        .pb_1()
        .px_2()
        .text_xs()
        .text_color(p.muted)
        .child(title.to_uppercase())
}
