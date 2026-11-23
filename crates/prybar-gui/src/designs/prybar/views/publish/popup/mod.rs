//! The create and edit popups: the same fields in three plain steps
//! (content, details, picture), then one primary button.

mod desc;
mod frame;
mod go;
mod picture;

use gpui::{div, prelude::*, px, AnyElement, Context, Entity};
use prybar_workshop::ListedItem;

use super::super::super::widgets;
use super::super::super::widgets::Tipped as _;
use super::{addon, fields};
use crate::app::{Forms, Root};
use crate::designs::input::TextInput;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(
    s: &AppState,
    f: &Forms,
    editing: Option<&ListedItem>,
    wide: bool,
    t: Tokens,
    cx: &Context<Root>,
) -> AnyElement {
    let fields = div().flex().flex_col().gap_3().flex_1().min_w_0();
    let fields = fields
        .child(content(s, f, editing, t, cx))
        .child(details(f, t));
    let heading = widgets::section("3 · Preview image", t).when(wide, |el| el.pt_0());
    let picture = div().flex().flex_col().gap_3().child(heading);
    let picture = picture.child(picture::view(s, f, editing, wide, t, cx));
    // Side by side the picture keeps a fixed column; stacked, the form
    // reads top to bottom on a narrow window.
    div()
        .debug_selector(|| "pb-pub-popup".into())
        .w_full()
        .flex()
        .when(wide, |el| el.items_start().gap_6())
        .when(!wide, |el| el.flex_col().gap_3())
        .child(fields)
        .child(picture.when(wide, |el| el.w(px(PICTURE_COLUMN)).flex_none()))
        .into_any_element()
}

/// The pinned footer: the last run's outcome above the two actions.
pub use go::footer;

/// Wide enough for the square and its notes under it.
const PICTURE_COLUMN: f32 = 240.;

const NOTE: &str = "Shown in the item's change history on the Workshop";
const TITLE: &str = "The name shown on the Workshop, up to 128 characters";
pub(super) const DESC: &str = "What it is and how to use it; Steam BBCode works";
const TAGS: &str = "Comma separated. Empty: the tags from addon.json, or the current ones";

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

/// Step 2: the words, who sees them and the note for this change.
fn details(f: &Forms, t: Tokens) -> impl IntoElement {
    let p = &f.publish;
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::section("2 · Details", t))
        .child(labeled("pb-pub-title", "Title", TITLE, &p.title, t))
        .child(desc::described(&p.editor, t))
        .child(labeled("pb-pub-tags", "Tags", TAGS, &p.tags, t))
        .child(fields::visibility(f, t))
        .child(labeled(
            "pb-pub-note",
            "Change note",
            NOTE,
            &p.change_note,
            t,
        ))
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
