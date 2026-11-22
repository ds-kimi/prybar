//! Form widgets: a labeled text field, a read-only value, a switch.

use gpui::{div, prelude::*, px, Div, ElementId, Entity, SharedString, Stateful};

use super::input::TextInput;
use super::ring;
use crate::theme::prybar::Tokens;

/// A labeled field; the label sits above, never inside.
pub fn field(label: &'static str, input: &Entity<TextInput>, t: Tokens) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_xs().text_color(t.muted).child(label))
        .child(input.clone())
}

/// A value the app chooses, shown like a disabled field: no border to
/// hover and no tab stop, so it never looks typeable.
pub fn readonly(label: &'static str, value: impl Into<SharedString>, t: Tokens) -> Div {
    let value = div()
        .h_8()
        .px_2()
        .flex()
        .items_center()
        .rounded_md()
        .bg(t.surface)
        .text_color(t.muted)
        .truncate()
        .child(value.into());
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_xs().text_color(t.muted).child(label))
        .child(value)
}

/// An on/off switch: the knob sits right and the track turns rust when on.
pub fn switch(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    on: bool,
    t: Tokens,
) -> Stateful<Div> {
    ring(div().id(id), t)
        .h_8()
        .px_1()
        .flex()
        .items_center()
        .gap_2()
        .rounded_md()
        .cursor_pointer()
        .hover(move |s| s.bg(t.hover))
        .active(move |s| s.bg(t.active))
        .child(track(on, t))
        .child(label.into())
}

fn track(on: bool, t: Tokens) -> Div {
    let knob = div()
        .size(px(12.))
        .rounded_full()
        .bg(if on { t.on_primary } else { t.muted });
    div()
        .w(px(28.))
        .h(px(16.))
        .px(px(2.))
        .flex()
        .items_center()
        .when(on, |el| el.justify_end())
        .rounded_full()
        .bg(if on { t.rust } else { t.active })
        .child(knob)
}
