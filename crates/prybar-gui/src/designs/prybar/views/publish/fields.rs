//! Form rows that are more than a labelled text field.

use gpui::{div, prelude::*, Context, Entity};

use super::super::super::widgets::Tipped as _;
use super::super::super::widgets::{self, Kind};
use super::addon;
use crate::app::{Forms, Root};
use crate::designs::input::TextInput;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

/// A path field with a Choose… button that fills it; `folder` adds a
/// second button for picking a folder.
pub fn picked(
    label: &'static str,
    id: &'static str,
    input: &Entity<TextInput>,
    folder: bool,
    t: Tokens,
    cx: &Context<Root>,
) -> impl IntoElement {
    let choose = |n: usize, text: &'static str, dirs: bool| {
        let target = input.clone();
        widgets::button((id, n), text, Kind::Secondary, true, t)
            .tip(
                if dirs {
                    "Pick the addon folder"
                } else {
                    "Pick a file"
                },
                t,
            )
            .on_click(cx.listener(move |this, _, _, cx| this.pick_into(target.clone(), dirs, cx)))
    };
    div()
        .flex()
        .items_end()
        .gap_2()
        .child(widgets::field(label, input, t).flex_1())
        .when(folder, |el| el.child(choose(0, "Folder…", true)))
        .child(choose(1, "File…", false))
}

pub fn visibility(f: &Forms, t: Tokens) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_xs().text_color(t.muted).child("Visibility"))
        .child(f.publish.visibility.clone())
}

/// The addon, its check right under it, then the preview image.
pub fn files(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let p = &f.publish;
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(picked(
            "Addon folder or .gma",
            "pb-pub-content",
            &p.content,
            true,
            t,
            cx,
        ))
        .child(addon::view(s, t))
        .child(picked(
            "Preview image",
            "pb-pub-preview",
            &p.preview,
            false,
            t,
            cx,
        ))
}
