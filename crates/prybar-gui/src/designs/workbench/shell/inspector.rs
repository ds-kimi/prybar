//! Right panel: facts about the selected file, whatever the screen.

use gpui::{div, prelude::*, AnyElement};

use super::super::widgets;
use crate::state::{AppState, Document};
use crate::theme::Palette;

pub fn panel(s: &AppState, p: Palette) -> AnyElement {
    let doc = s.selected.and_then(|i| s.documents.get(i));
    let rows = match doc {
        Some(doc) => facts(doc),
        None => vec![("Selection".into(), "nothing open".into())],
    };
    let body = rows.into_iter().map(|(k, v)| kv(k, v, p));
    div()
        .id("wb-inspector")
        .debug_selector(|| "wb-inspector".into())
        .w_64()
        .flex_none()
        .p_2()
        .flex()
        .flex_col()
        .overflow_y_scroll()
        .bg(p.surface)
        .border_l_1()
        .border_color(p.border)
        .child(widgets::section("Inspector", p))
        .children(body)
        .children(doc.map(|d| companions(d, p)))
        .into_any_element()
}

fn facts(doc: &Document) -> Vec<(String, String)> {
    let row = |k: &str, v: String| (k.to_owned(), v);
    match doc {
        Document::Model { summary: m, .. } => vec![
            row("Name", m.name.clone()),
            row("Version", m.version.to_string()),
            row("Bones", m.bones.to_string()),
            row("Meshes", m.meshes.to_string()),
            row("Sequences", m.sequences.to_string()),
            row("Materials", m.materials.len().to_string()),
            row("Flex", m.flex_controllers.to_string()),
            row("Checksum", format!("{:08x}", m.checksum)),
        ],
        Document::Package { report: r, path } => vec![
            row("Format", format!("{:?}", r.format)),
            row("Entries", r.total.to_string()),
            row("Size", format!("{} B", r.total_bytes)),
            row("Path", path.display().to_string()),
        ],
    }
}

fn kv(k: String, v: String, p: Palette) -> impl IntoElement {
    div()
        .px_2()
        .py_0p5()
        .flex()
        .gap_2()
        .child(div().w_20().flex_none().text_color(p.muted).child(k))
        .child(div().min_w_0().overflow_hidden().child(v))
}

fn companions(doc: &Document, p: Palette) -> AnyElement {
    let rows = doc.companions().into_iter().map(|(suffix, ok)| {
        let dot = div()
            .size_1p5()
            .rounded_full()
            .bg(if ok { p.accent } else { p.danger });
        div()
            .px_2()
            .py_0p5()
            .flex()
            .items_center()
            .gap_2()
            .child(dot)
            .child(suffix)
            .child(
                div()
                    .text_color(p.muted)
                    .child(if ok { "found" } else { "missing" }),
            )
    });
    div().flex().flex_col().children(rows).into_any_element()
}
