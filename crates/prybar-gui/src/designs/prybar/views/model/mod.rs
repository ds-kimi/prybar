//! Model screen: the facts in one aligned block, companions, then the
//! material lists. Previews hang off the header since they need a model.

mod facts;

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::widgets::{self, Kind};
use crate::app::Root;
use crate::state::{Document, Screen};
use crate::theme::prybar::Tokens;
use facts::{companions, facts, list};

pub fn view(doc: &Document, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let Document::Model { summary: m, .. } = doc else {
        return div().into_any_element();
    };
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(head(doc, t, cx))
        .child(facts(m, t))
        .child(widgets::section("Companion files", t))
        .child(companions(doc, t))
        .child(widgets::section("Materials", t))
        .child(list(&m.materials, "This model names no materials.", t))
        .child(widgets::section("Material folders", t))
        .child(list(&m.material_dirs, "No material folders.", t))
        .children((!m.notes.is_empty()).then(|| widgets::section("Not recovered", t)))
        .children((!m.notes.is_empty()).then(|| list(&m.notes, "", t)))
        .into_any_element()
}

fn head(doc: &Document, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let go = |id: &'static str, screen: Screen| {
        widgets::button(id, screen.label(), Kind::Secondary, true, t)
            .debug_selector(move || id.into())
            .on_click(cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| s.navigate(screen, cx))
            }))
    };
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(widgets::heading(doc.title()).flex_1())
        .child(go("pb-to-3d", Screen::ModelPreview))
        .child(go("pb-to-tex", Screen::TexturePreview))
}
