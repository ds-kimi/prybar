//! The description field inside its BBCode editor.

use gpui::{div, prelude::*, Entity};

use super::super::super::super::widgets::Tipped as _;
use super::DESC;
use crate::designs::bbcode::BbEditor;
use crate::theme::prybar::Tokens;

/// The description sits in its BBCode editor; the field inside keeps
/// the selector tests look for.
pub fn described(editor: &Entity<BbEditor>, t: Tokens) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .id(("pb-pub-desc", 0usize))
                .text_xs()
                .text_color(t.muted)
                .child("Description")
                .tip(DESC, t),
        )
        .child(
            div()
                .debug_selector(|| "pb-pub-desc".into())
                .child(editor.clone()),
        )
}
