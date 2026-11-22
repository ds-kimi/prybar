//! 3D and texture preview screens: a toolbar over the later-work viewport.

use gpui::{div, prelude::*, AnyElement};

use super::super::super::placeholder;
use super::super::widgets;
use crate::state::{Document, Placeholder};
use crate::theme::Palette;

/// 3D or texture preview: a toolbar over the placeholder viewport.
pub fn preview(ph: Placeholder, doc: Option<&Document>, p: Palette) -> AnyElement {
    let name = doc.map_or_else(|| "no file".into(), |d| d.title());
    let tools = ["Orbit", "Wireframe", "Reset"]
        .into_iter()
        .map(|t| widgets::button(t, t, false, doc.is_some(), p));
    div()
        .size_full()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(div().flex_1().text_color(p.muted).child(name))
                .children(tools),
        )
        .child(placeholder::region(ph, p))
        .into_any_element()
}
