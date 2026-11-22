//! Model screen: the live 3D view beside the facts, companions and
//! material lists, so opening a model shows it at once.

mod facts;
mod sequences;

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::widgets::Tipped as _;
use super::super::widgets::{self, Kind};
use crate::app::{Forms, Root};
use crate::state::{Document, Screen};
use crate::theme::prybar::Tokens;
use facts::{companions, facts, list};
use sequences::sequences;

pub fn view(doc: &Document, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let Document::Model { summary: m, .. } = doc else {
        return div().into_any_element();
    };
    // Fixed height: the screen scrolls, and a flex child of a scroll
    // area would collapse to nothing.
    let view = div().flex_1().min_w_64().h(gpui::px(520.0)).flex();
    let row = div().flex().items_start().gap_4();
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(head(doc, t, cx))
        .child(
            row.child(details(doc, m, f, t, cx))
                .child(view.child(f.viewport.clone())),
        )
        .into_any_element()
}

fn details(
    doc: &Document,
    m: &prybar_mdl::Summary,
    f: &Forms,
    t: Tokens,
    cx: &Context<Root>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .w(gpui::px(380.0))
        .flex_none()
        .child(facts(m, t))
        .child(widgets::section("Sequences", t))
        .child(sequences(f, t, cx))
        .child(widgets::section("Companion files", t))
        .child(companions(doc, t))
        .child(widgets::section("Materials", t))
        .child(list(&m.materials, "This model names no materials.", t))
        .child(widgets::section("Material folders", t))
        .child(list(&m.material_dirs, "No material folders.", t))
        .children((!m.notes.is_empty()).then(|| widgets::section("Not recovered", t)))
        .children((!m.notes.is_empty()).then(|| list(&m.notes, "", t)))
}

fn head(doc: &Document, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let go = |id: &'static str, screen: Screen| {
        widgets::button(id, screen.label(), Kind::Secondary, true, t)
            .debug_selector(move || id.into())
            .tip(
                format!(
                    "Open the {} screen for this model",
                    screen.label().to_lowercase()
                ),
                t,
            )
            .on_click(cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| s.navigate(screen, cx))
            }))
    };
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(widgets::heading(doc.title()).flex_1())
        .child(go("pb-to-tex", Screen::TexturePreview))
}
