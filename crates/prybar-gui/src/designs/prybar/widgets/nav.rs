//! Top bar entries.

use gpui::{div, prelude::*, px, AnyElement, ElementId, FontWeight, SharedString, Stateful};

use super::super::motion;
use super::ring;
use crate::theme::prybar::Tokens;

/// A top bar entry. The current one gets full-contrast text and a rust
/// underline; the rest are muted until hovered.
pub fn nav_item(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    current: bool,
    t: Tokens,
) -> Stateful<gpui::Div> {
    let label: SharedString = label.into();
    let el = div()
        .id(id)
        .relative()
        .h_7()
        .px_2p5()
        .flex()
        .items_center()
        .rounded_md()
        .cursor_pointer()
        .text_color(if current { t.text } else { t.muted })
        .when(current, |el| el.font_weight(FontWeight::MEDIUM))
        .hover(move |s| s.bg(t.hover).text_color(t.text))
        .active(move |s| s.bg(t.active))
        .when(current, |el| el.child(mark(&label, t)))
        .child(label);
    ring(el, t)
}

/// Keyed by label so moving to another entry draws a fresh line there.
fn mark(label: &str, t: Tokens) -> AnyElement {
    let line = div()
        .absolute()
        .bottom(px(-10.))
        .h(px(2.))
        .rounded_sm()
        .bg(t.rust);
    motion::underline(line, SharedString::from(format!("pb-mark-{label}")))
}
