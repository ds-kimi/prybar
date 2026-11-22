//! Publish, the preview screens and the decompile result.

use gpui::{div, prelude::*, AnyElement, Div};

use super::super::super::placeholder;
use super::super::widgets;
use crate::state::{AppState, Document, Placeholder};
use crate::theme::Palette;

/// `key  value` line; text entry is later work.
fn field(k: &str, v: String, p: Palette) -> Div {
    div()
        .flex()
        .child(div().w_32().text_color(p.muted).child(k.to_owned()))
        .child(
            div()
                .border_b_1()
                .border_color(p.border)
                .min_w_64()
                .child(v),
        )
}

pub fn publish(s: &AppState, p: Palette) -> AnyElement {
    let title = s
        .selected
        .and_then(|i| s.documents.get(i))
        .map_or_else(|| "untitled".into(), |d| d.title());
    div()
        .size_full()
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("publish", "workshop item", p))
        .child(field("title", title, p))
        .child(field("visibility", "private".into(), p))
        .child(field("tags", "model".into(), p))
        .child(
            div()
                .flex()
                .gap_3()
                .child(widgets::action(
                    "cs-pub-dry",
                    "dry run",
                    "publish --dry-run",
                    false,
                    p,
                ))
                .child(widgets::action("cs-pub-go", "publish", "publish", false, p)),
        )
        .child(placeholder::region(Placeholder::PublishProgress, p))
        .into_any_element()
}

pub fn preview(ph: Placeholder, doc: Option<&Document>, p: Palette) -> AnyElement {
    let name = doc.map_or_else(|| "no file".into(), |d| d.title());
    div()
        .size_full()
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("preview", name, p))
        .child(placeholder::region(ph, p))
        .into_any_element()
}

/// One line per decompiled model, failures in red.
pub fn result(s: &AppState, p: Palette) -> impl IntoElement {
    let lines = s.last_decompile.iter().flat_map(|r| {
        let ok = r.models.iter().map(|m| {
            let text = format!(
                "ok   {} → {} files in {}",
                m.input,
                m.files,
                m.out_dir.display()
            );
            div().text_color(p.accent).child(text)
        });
        let bad = r.failures.iter().map(|f| {
            div()
                .text_color(p.danger)
                .child(format!("fail {}: {}", f.input, f.error))
        });
        ok.chain(bad).collect::<Vec<_>>()
    });
    div().flex().flex_col().children(lines)
}
