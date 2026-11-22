//! Workshop publish form; upload itself is later work.

use gpui::{div, prelude::*, px, AnyElement};

use super::super::widgets::{self, Kind};
use super::preview::region;
use crate::state::{AppState, Placeholder};
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, t: Tokens) -> AnyElement {
    let title = s
        .selected
        .and_then(|i| s.documents.get(i))
        .map_or_else(|| "Untitled addon".into(), |d| d.title());
    div()
        .size_full()
        .flex()
        .flex_col()
        .gap_4()
        .child(form(title, t))
        .child(region(Placeholder::PublishProgress, t))
        .into_any_element()
}

/// Buttons stay disabled until the upload is built.
fn form(title: String, t: Tokens) -> impl IntoElement {
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
        .child(widgets::field("pb-pub-title", "Title", title, t))
        .child(widgets::field("pb-pub-vis", "Visibility", "Private", t))
        .child(widgets::field("pb-pub-tags", "Tags", "model", t))
        .child(
            div()
                .flex()
                .gap_2()
                .child(off("pb-pub-dry", "Dry run", Kind::Secondary))
                .child(off("pb-pub-go", "Publish", Kind::Primary)),
        )
}
