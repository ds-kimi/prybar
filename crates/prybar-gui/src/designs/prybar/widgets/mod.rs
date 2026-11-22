//! Building blocks. Each interactive one has hover, press, focus-visible
//! and disabled states drawn from the tokens.

mod button;
mod field;
pub mod input;
mod nav;
pub mod tip;

pub use button::{button, Kind};
pub use field::{field, readonly, switch};
pub use nav::nav_item;
pub use tip::Tipped;

use gpui::{div, prelude::*, Div, ElementId, FontWeight, SharedString, Stateful};

use crate::theme::prybar::Tokens;

/// Focus ring for keyboard users only: rust, never shown on click.
pub fn ring(el: Stateful<Div>, t: Tokens) -> Stateful<Div> {
    el.tab_index(0)
        .border_1()
        .border_color(gpui::transparent_black())
        .focus_visible(move |s| s.border_color(t.rust))
}

/// A list row: tinted on hover, darker while pressed, rust-marked text
/// when selected so selection never relies on a fill alone.
pub fn row(id: impl Into<ElementId>, selected: bool, t: Tokens) -> Stateful<Div> {
    let el = div()
        .id(id)
        .h_8()
        .px_2()
        .flex()
        .items_center()
        .gap_2()
        .rounded_md()
        .cursor_pointer()
        .text_color(if selected { t.rust } else { t.text })
        .when(selected, |el| el.bg(t.hover))
        .hover(move |s| s.bg(t.hover))
        .active(move |s| s.bg(t.active));
    ring(el, t)
}

/// Screen title: one size up from body, weight does the rest.
pub fn heading(text: impl Into<SharedString>) -> Div {
    div()
        .text_base()
        .font_weight(FontWeight::SEMIBOLD)
        .child(text.into())
}

/// A group label inside a screen: sentence case, muted, never caps.
pub fn section(text: impl Into<SharedString>, t: Tokens) -> Div {
    div()
        .pt_2()
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .text_color(t.muted)
        .child(text.into())
}
