//! The description: a tall field that takes BBCode, and its editor.

use std::sync::Arc;

use gpui::{App, AppContext as _, Entity};

use crate::designs::bbcode::BbEditor;
use crate::designs::input::{Spec, TextInput};
use crate::state::checks;
use crate::state::tools::default_thumb;

/// Several lines from the start, since descriptions run long; it grows
/// with the text, then scrolls inside.
pub fn field(cx: &mut App) -> Entity<TextInput> {
    let spec = Spec {
        placeholder: "What it is and how to use it",
        multiline: true,
        text: String::new(),
        check: checks::description,
        rows: (6, 16),
        markup: true,
    };
    cx.new(|cx| TextInput::new(spec, cx))
}

/// Images in the preview come through the item preview cache.
pub fn editor(input: &Entity<TextInput>, cx: &mut App) -> Entity<BbEditor> {
    let input = input.clone();
    cx.new(|cx| BbEditor::new(input, Arc::new(default_thumb), cx))
}
