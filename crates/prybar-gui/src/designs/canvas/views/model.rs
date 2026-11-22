//! Model: big number cards that ripple in, then materials as chips and
//! the companion files with a found/missing mark.

use gpui::{div, prelude::*, AnyElement};

use super::super::{motion, widgets};
use crate::state::Document;
use crate::theme::Palette;

pub fn view(doc: &Document, p: Palette) -> AnyElement {
    let Document::Model { summary: m, .. } = doc else {
        return div().into_any_element();
    };
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(widgets::title(
            "Model",
            format!("{} · version {}", m.name, m.version),
            p,
        ))
        .child(stats(m, p))
        .child(lists(doc, &m.materials, p))
        .children((!m.notes.is_empty()).then(|| notes(&m.notes, p)))
        .into_any_element()
}

fn stats(m: &prybar_mdl::Summary, p: Palette) -> impl IntoElement {
    let cards = [
        ("Bones", m.bones.to_string()),
        ("Meshes", m.meshes.to_string()),
        ("Sequences", m.sequences.to_string()),
        ("Materials", m.materials.len().to_string()),
        ("Flex", m.flex_controllers.to_string()),
        ("Body parts", m.body_parts.to_string()),
    ];
    let cards = cards.into_iter().enumerate().map(|(ix, (k, v))| {
        let card = widgets::card(k, p)
            .w_40()
            .flex()
            .flex_col()
            .gap_1()
            .child(div().text_3xl().text_color(p.accent).child(v))
            .child(div().text_sm().text_color(p.muted).child(k));
        motion::enter(div().child(card), ("cv-stat", ix), ix)
    });
    div().flex().flex_wrap().gap_4().children(cards)
}

fn notes(items: &[String], p: Palette) -> AnyElement {
    let rows = items.iter().map(|t| div().child(format!("• {t}")));
    div()
        .p_4()
        .rounded_2xl()
        .bg(p.hover)
        .flex()
        .flex_col()
        .gap_1()
        .text_sm()
        .children(rows)
        .into_any_element()
}

/// Materials as chips, then companions: found ones filled, missing crossed.
fn lists(doc: &Document, materials: &[String], p: Palette) -> impl IntoElement {
    let mats = materials.iter().map(|t| widgets::chip(t.clone(), false, p));
    let comps = doc.companions().into_iter().map(|(ext, ok)| {
        let mark = if ok { "✓" } else { "✗" };
        widgets::chip(format!("{mark} {ext}"), ok, p)
    });
    let hint = "Files that must sit next to the .mdl.";
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(widgets::title(
            "Materials",
            "Textures this model asks for.",
            p,
        ))
        .child(div().flex().flex_wrap().gap_2().children(mats))
        .child(widgets::title("Companions", hint, p))
        .child(div().flex().flex_wrap().gap_2().children(comps))
}
