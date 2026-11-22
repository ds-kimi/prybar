//! Buttons: primary in the strong neutral, secondary outlined, ghost
//! bare until hovered.

use gpui::{div, prelude::*, Div, ElementId, FontWeight, SharedString, Stateful};

use super::ring;
use crate::theme::prybar::Tokens;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Primary,
    Secondary,
    Ghost,
}

/// Disabled buttons keep their place and contrast-safe text but take no
/// input, so callers attach `on_click` only when `enabled`.
pub fn button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    kind: Kind,
    enabled: bool,
    t: Tokens,
) -> Stateful<Div> {
    let (bg, fg) = match kind {
        Kind::Primary => (t.primary, t.on_primary),
        Kind::Secondary | Kind::Ghost => (gpui::transparent_black().into(), t.text),
    };
    let el = ring(div().id(id), t)
        .h_8()
        .px_3()
        .flex()
        .items_center()
        .rounded_md()
        .bg(bg)
        .text_color(fg)
        .font_weight(FontWeight::MEDIUM)
        .when(kind == Kind::Secondary, |el| el.border_color(t.border))
        .child(label.into());
    if !enabled {
        return el.text_color(t.muted).bg(t.hover).border_color(t.hover);
    }
    live(el.cursor_pointer(), kind, t)
}

fn live(el: Stateful<Div>, kind: Kind, t: Tokens) -> Stateful<Div> {
    if kind == Kind::Primary {
        return el.hover(|s| s.opacity(0.88)).active(|s| s.opacity(0.74));
    }
    el.hover(move |s| s.bg(t.hover))
        .active(move |s| s.bg(t.active))
}
