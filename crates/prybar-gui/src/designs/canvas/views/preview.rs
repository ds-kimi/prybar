//! 3D and texture preview: tool pills over the later-work viewport.

use gpui::{div, prelude::*, AnyElement};

use super::super::super::placeholder;
use super::super::widgets;
use crate::state::{Document, Placeholder};
use crate::theme::Palette;

pub fn view(ph: Placeholder, doc: Option<&Document>, p: Palette) -> AnyElement {
    let name = doc.map_or_else(|| "No file open".into(), |d| d.title());
    let tools = ["Spin", "Wireframe", "Reset"]
        .into_iter()
        .map(|t| widgets::pill(t, t, false, doc.is_some(), p));
    div()
        .size_full()
        .flex()
        .flex_col()
        .gap_4()
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(div().flex_1().child(widgets::title("Preview", name, p)))
                .children(tools),
        )
        .child(placeholder::region(ph, p))
        .into_any_element()
}
