//! Workshop publish: a soft form over the later-work upload progress.

use gpui::{div, prelude::*, AnyElement, Div};

use super::super::super::placeholder;
use super::super::widgets;
use crate::state::{AppState, Placeholder};
use crate::theme::Palette;

/// A labeled value in a rounded box; text entry is later work.
fn field(label: &'static str, value: String, p: Palette) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_sm().text_color(p.muted).child(label))
        .child(
            div()
                .px_4()
                .py_3()
                .rounded_2xl()
                .bg(p.surface)
                .border_1()
                .border_color(p.border)
                .child(value),
        )
}

pub fn publish(s: &AppState, p: Palette) -> AnyElement {
    let title = s
        .selected
        .and_then(|i| s.documents.get(i))
        .map_or_else(|| "Untitled".into(), |d| d.title());
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(widgets::title(
            "Share on the Workshop",
            "Fill this in, try a dry run, then publish.",
            p,
        ))
        .child(form(title, p))
        .child(
            div()
                .flex()
                .gap_3()
                .child(widgets::pill("cv-pub-dry", "Dry run", false, false, p))
                .child(widgets::pill("cv-pub-go", "Publish", true, false, p)),
        )
        .child(placeholder::region(Placeholder::PublishProgress, p))
        .into_any_element()
}

fn form(title: String, p: Palette) -> Div {
    div()
        .max_w_96()
        .flex()
        .flex_col()
        .gap_3()
        .child(field("Title", title, p))
        .child(field("Who can see it", "Only me".into(), p))
        .child(field("Tags", "model".into(), p))
}
