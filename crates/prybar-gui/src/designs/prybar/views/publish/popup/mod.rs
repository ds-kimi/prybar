//! The create and edit popups: the same fields in three plain steps
//! (content, details, picture), then one primary button.

mod go;
mod picture;

use gpui::{div, prelude::*, px, AnyElement, Context, Entity};
use prybar_workshop::ListedItem;

use super::super::super::widgets;
use super::super::super::widgets::Tipped as _;
use super::{addon, fields, result};
use crate::app::{Forms, Root};
use crate::designs::input::TextInput;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(
    s: &AppState,
    f: &Forms,
    editing: Option<&ListedItem>,
    t: Tokens,
    cx: &Context<Root>,
) -> AnyElement {
    div()
        .debug_selector(|| "pb-pub-popup".into())
        .w(px(560.))
        .flex()
        .flex_col()
        .gap_3()
        .child(content(s, f, editing, t, cx))
        .child(details(f, t))
        .child(widgets::section("3 · Preview image", t))
        .child(picture::view(s, f, editing, t, cx))
        .child(labeled(
            "pb-pub-note",
            "Change note",
            NOTE,
            &f.publish.change_note,
            t,
        ))
        .child(go::buttons(s, f, editing.is_some(), t, cx))
        .child(result::view(s, t, cx))
        .into_any_element()
}

const NOTE: &str = "Shown in the item's change history on the Workshop";

/// Step 1: what is uploaded, checked as soon as it is picked.
fn content(
    s: &AppState,
    f: &Forms,
    editing: Option<&ListedItem>,
    t: Tokens,
    cx: &Context<Root>,
) -> impl IntoElement {
    let title = match editing {
        Some(_) => "1 · New content (optional)",
        None => "1 · Content",
    };
    let p = &f.publish;
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::section(title, t).pt_0())
        .child(fields::picked(
            "Addon folder or .gma",
            "pb-pub-content",
            &p.content,
            true,
            t,
            cx,
        ))
        .child(addon::view(s, t))
        .when_some(editing, |el, it| el.child(go::link_hint(s, it, t)))
}

/// Step 2: the words and who sees them.
fn details(f: &Forms, t: Tokens) -> impl IntoElement {
    let p = &f.publish;
    let tags = "Comma separated. Empty: the tags from addon.json, or the current ones";
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::section("2 · Details", t))
        .child(labeled(
            "pb-pub-title",
            "Title",
            "The name shown on the Workshop, up to 128 characters",
            &p.title,
            t,
        ))
        .child(labeled(
            "pb-pub-desc",
            "Description",
            "What it is and how to use it; Steam BBCode works",
            &p.description,
            t,
        ))
        .child(labeled("pb-pub-tags", "Tags", tags, &p.tags, t))
        .child(fields::visibility(f, t))
}

/// A field whose label explains it on hover; the input keeps the
/// selector tests and the IME code look for.
fn labeled(
    id: &'static str,
    label: &'static str,
    tip: &'static str,
    input: &Entity<TextInput>,
    t: Tokens,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .id((id, 0usize))
                .text_xs()
                .text_color(t.muted)
                .child(label)
                .tip(tip, t),
        )
        .child(div().debug_selector(move || id.into()).child(input.clone()))
}
