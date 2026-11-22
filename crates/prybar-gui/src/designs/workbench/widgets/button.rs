//! Buttons: primary in the accent, secondary on the surface.

use gpui::{div, prelude::*, Div, ElementId, SharedString, Stateful};

use super::ring;
use crate::theme::Palette;

/// A compact button. Disabled ones keep their place but take no input,
/// so the caller only attaches `on_click` when `enabled`.
pub fn button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    primary: bool,
    enabled: bool,
    p: Palette,
) -> Stateful<Div> {
    let (bg, fg, edge) = if primary {
        (p.accent, p.bg, p.accent)
    } else {
        (p.surface, p.text, p.border)
    };
    let el = div()
        .id(id)
        .h_7()
        .px_3()
        .flex()
        .items_center()
        .rounded_md()
        .border_1()
        .border_color(edge)
        .bg(bg)
        .text_color(fg)
        .child(label.into());
    if !enabled {
        return el.opacity(0.4);
    }
    live(ring(el, p), primary, p)
}

/// Primary buttons fade on hover and press; secondary ones tint.
fn live(el: Stateful<Div>, primary: bool, p: Palette) -> Stateful<Div> {
    let el = el.cursor_pointer();
    if primary {
        return el.hover(|s| s.opacity(0.9)).active(|s| s.opacity(0.75));
    }
    el.hover(move |s| s.bg(p.hover))
        .active(move |s| s.bg(p.active))
}
