//! Form widgets. Text entry is later work, so fields show their value
//! and still take focus and hover like a real input.

use gpui::{div, prelude::*, Div, ElementId, SharedString, Stateful};

use super::ring;
use crate::theme::Palette;

/// A labeled single-line field.
pub fn field(
    id: impl Into<ElementId>,
    label: &'static str,
    value: impl Into<SharedString>,
    p: Palette,
) -> Div {
    let input = div()
        .id(id)
        .h_7()
        .px_2()
        .flex()
        .items_center()
        .rounded_md()
        .border_1()
        .border_color(p.border)
        .bg(p.bg)
        .hover(move |s| s.border_color(p.muted))
        .child(value.into());
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_xs().text_color(p.muted).child(label))
        .child(ring(input, p))
}

/// A checkbox row; the box fills with the accent when on.
pub fn toggle(
    id: impl Into<ElementId>,
    label: &'static str,
    on: bool,
    p: Palette,
) -> Stateful<Div> {
    let mark = div()
        .size_3p5()
        .rounded_sm()
        .border_1()
        .border_color(if on { p.accent } else { p.muted })
        .when(on, |el| el.bg(p.accent));
    let el = div()
        .id(id)
        .h_7()
        .px_1()
        .flex()
        .items_center()
        .gap_2()
        .rounded_md()
        .border_1()
        .border_color(gpui::transparent_black())
        .cursor_pointer()
        .hover(move |s| s.bg(p.hover))
        .active(move |s| s.bg(p.active))
        .child(mark)
        .child(label);
    ring(el, p)
}
