//! 3D and texture previews, and the calm region every later-work part
//! uses: a labeled frame, no fake art, no motion.

use gpui::{div, prelude::*, AnyElement};

use super::super::widgets;
use crate::state::{Document, Placeholder, Screen};
use crate::theme::prybar::Tokens;

pub fn view(screen: Screen, doc: Option<&Document>, t: Tokens) -> AnyElement {
    let name = doc.map_or_else(|| "No model open".into(), |d| d.title());
    let ph = screen.placeholder().unwrap_or(Placeholder::ModelViewport);
    div()
        .size_full()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .items_baseline()
                .gap_3()
                .child(widgets::heading(screen.label()))
                .child(div().text_color(t.muted).child(name)),
        )
        .child(region(ph, t))
        .into_any_element()
}

/// Says what will live here and that it is not built yet.
pub fn region(ph: Placeholder, t: Tokens) -> AnyElement {
    div()
        .debug_selector(|| "pb-placeholder".into())
        .flex_1()
        .min_h_48()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_1()
        .rounded_lg()
        .border_1()
        .border_color(t.border)
        .bg(t.surface)
        .child(ph.label())
        .child(
            div()
                .text_xs()
                .text_color(t.muted)
                .child("Coming in a later release"),
        )
        .into_any_element()
}
