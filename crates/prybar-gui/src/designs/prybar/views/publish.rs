//! Workshop publish form; upload itself is later work.

use gpui::{div, prelude::*, px, AnyElement};

use super::super::widgets::{self, Kind};
use super::preview::region;
use crate::app::Forms;
use crate::state::Placeholder;
use crate::theme::prybar::Tokens;

pub fn view(f: &Forms, t: Tokens) -> AnyElement {
    div()
        .size_full()
        .flex()
        .flex_col()
        .gap_4()
        .child(form(f, t))
        .child(region(Placeholder::PublishProgress, t))
        .into_any_element()
}

/// Buttons stay disabled until the upload is built.
fn form(f: &Forms, t: Tokens) -> impl IntoElement {
    let off =
        |id: &'static str, label: &'static str, kind| widgets::button(id, label, kind, false, t);
    div()
        .max_w(px(520.))
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("Publish"))
        .child(
            div()
                .text_color(t.muted)
                .child("Upload an addon to the Steam Workshop."),
        )
        .child(widgets::field("Title", &f.title, t).debug_selector(|| "pb-pub-title".into()))
        .child(widgets::field("Description", &f.description, t))
        .child(widgets::field("Tags", &f.tags, t))
        .child(widgets::field("Visibility", &f.visibility, t))
        .child(
            div()
                .flex()
                .gap_2()
                .child(off("pb-pub-dry", "Dry run", Kind::Secondary))
                .child(off("pb-pub-go", "Publish", Kind::Primary)),
        )
}
