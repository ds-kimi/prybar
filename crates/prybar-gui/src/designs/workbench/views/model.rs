//! Model screen: counts up top, then materials and notes.

use gpui::{div, prelude::*, AnyElement};

use super::super::widgets;
use crate::state::Document;
use crate::theme::Palette;

pub fn view(doc: &Document, p: Palette) -> AnyElement {
    let Document::Model { summary: m, .. } = doc else {
        return div().into_any_element();
    };
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_lg()
                .child(format!("{} · v{}", m.name, m.version)),
        )
        .child(stats(m, p))
        .child(widgets::section("Materials", p))
        .child(list(&m.materials, "No materials", p))
        .child(widgets::section("Material folders", p))
        .child(list(&m.material_dirs, "None", p))
        .children((!m.notes.is_empty()).then(|| widgets::section("Notes", p)))
        .children((!m.notes.is_empty()).then(|| list(&m.notes, "", p)))
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
    div()
        .flex()
        .flex_wrap()
        .gap_2()
        .children(cards.into_iter().map(|(k, v)| stat(k, v, p)))
}

fn stat(label: &'static str, value: String, p: Palette) -> impl IntoElement {
    div()
        .id(label)
        .w_32()
        .p_2()
        .flex()
        .flex_col()
        .rounded_md()
        .border_1()
        .border_color(p.border)
        .bg(p.surface)
        .hover(move |st| st.border_color(p.muted))
        .child(div().text_lg().child(value))
        .child(div().text_xs().text_color(p.muted).child(label))
}

fn list(items: &[String], none: &'static str, p: Palette) -> AnyElement {
    if items.is_empty() {
        return div()
            .px_2()
            .text_color(p.muted)
            .child(none)
            .into_any_element();
    }
    let rows = items.iter().map(|t| div().px_2().py_0p5().child(t.clone()));
    div().flex().flex_col().children(rows).into_any_element()
}
