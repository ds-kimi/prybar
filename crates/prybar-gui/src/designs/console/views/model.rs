//! Model: a key/value table, then materials and companions as lists.

use gpui::{div, prelude::*, AnyElement, Div};

use super::super::widgets;
use crate::state::Document;
use crate::theme::Palette;

pub fn view(doc: &Document, p: Palette) -> AnyElement {
    let Document::Model { summary: m, .. } = doc else {
        return div().into_any_element();
    };
    let table = facts(m).into_iter().map(|(k, v)| kv(k, v, p));
    let mats = m.materials.iter().map(|t| div().child(format!("  {t}")));
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("model", m.name.clone(), p))
        .child(div().flex().flex_col().children(table))
        .child(widgets::heading("materials", "textures it asks for", p))
        .child(div().flex().flex_col().children(mats))
        .child(companions(doc, p))
        .children(
            m.notes
                .iter()
                .map(|n| div().text_color(p.muted).child(format!("# {n}"))),
        )
        .into_any_element()
}

fn facts(m: &prybar_mdl::Summary) -> [(&'static str, String); 7] {
    [
        ("version", m.version.to_string()),
        ("bones", m.bones.to_string()),
        ("meshes", m.meshes.to_string()),
        ("sequences", m.sequences.to_string()),
        ("materials", m.materials.len().to_string()),
        ("flex", m.flex_controllers.to_string()),
        ("bodyparts", m.body_parts.to_string()),
    ]
}

fn kv(k: &str, v: String, p: Palette) -> Div {
    div()
        .flex()
        .child(div().w_32().text_color(p.muted).child(k.to_owned()))
        .child(div().text_color(p.accent).child(v))
}

/// `ok` or `missing` per companion file, missing in red.
fn companions(doc: &Document, p: Palette) -> Div {
    let rows = doc.companions().into_iter().map(|(ext, ok)| {
        let (mark, color) = if ok {
            ("ok", p.accent)
        } else {
            ("missing", p.danger)
        };
        div()
            .flex()
            .child(div().w_32().child(ext))
            .child(div().text_color(color).child(mark))
    });
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(widgets::heading(
            "companions",
            "must sit next to the .mdl",
            p,
        ))
        .children(rows)
}
