//! The model's numbers, companions and plain lists.

use gpui::{div, prelude::*, px, AnyElement};
use prybar_mdl::Summary;

use crate::state::Document;
use crate::theme::prybar::Tokens;

/// Label/value rows in two aligned columns: easier to scan than cards.
pub fn facts(m: &Summary, t: Tokens) -> impl IntoElement {
    let rows = [
        ("Internal name", m.name.clone()),
        ("Version", format!("{} ({:?})", m.version, m.family)),
        ("Bones", m.bones.to_string()),
        ("Meshes", m.meshes.to_string()),
        ("Sequences", m.sequences.to_string()),
        ("Materials", m.materials.len().to_string()),
        ("Flex controllers", m.flex_controllers.to_string()),
        ("Body parts", m.body_parts.to_string()),
    ];
    let row = move |(k, v): (&'static str, String)| {
        div()
            .flex()
            .gap_4()
            .child(div().w(px(140.)).text_color(t.muted).child(k))
            .child(div().child(v))
    };
    div()
        .debug_selector(|| "pb-facts".into())
        .flex()
        .flex_col()
        .gap_1()
        .children(rows.into_iter().map(row))
}

/// Found ones in plain text, missing ones in danger with the word, so
/// the state never depends on color alone.
pub fn companions(doc: &Document, t: Tokens) -> impl IntoElement {
    let items = doc.companions().into_iter().map(move |(ext, found)| {
        let (text, color) = if found {
            (format!("{ext} found"), t.text)
        } else {
            (format!("{ext} missing"), t.danger)
        };
        div().text_color(color).child(text)
    });
    div().flex().flex_wrap().gap_4().children(items)
}

pub fn list(items: &[String], none: &'static str, t: Tokens) -> AnyElement {
    if items.is_empty() {
        return div().text_color(t.muted).child(none).into_any_element();
    }
    let rows = items.iter().map(|s| div().py_0p5().child(s.clone()));
    div().flex().flex_col().children(rows).into_any_element()
}
