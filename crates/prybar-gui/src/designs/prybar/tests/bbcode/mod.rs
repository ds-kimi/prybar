//! The description editor: toolbar, shortcuts, soft wrap and preview.

mod editor;
mod preview;

use gpui::{Entity, Focusable as _, Modifiers, VisualTestContext};

use super::{details::new_popup, forms, window};
use crate::designs::input::TextInput;

/// The new item popup open, the description holding `text`, focused,
/// with the caret at its end.
fn description(
    cx: &mut gpui::TestAppContext,
    text: &str,
) -> (VisualTestContext, Entity<TextInput>) {
    let (state, mut cx) = window(cx, false);
    new_popup(&state, &mut cx);
    let desc = forms(&mut cx).publish.description;
    desc.update(&mut cx, |i, cx| i.set_text(text, cx));
    cx.update(|window, cx| desc.read(cx).focus_handle(cx).focus(window, cx));
    cx.run_until_parked();
    (cx, desc)
}

fn click(cx: &mut VisualTestContext, id: &'static str) {
    let b = cx.debug_bounds(id).unwrap_or_else(|| panic!("{id} drawn"));
    cx.simulate_click(b.center(), Modifiers::none());
    cx.run_until_parked();
}

fn text(cx: &mut VisualTestContext, desc: &Entity<TextInput>) -> String {
    desc.read_with(cx, |i, _| i.text().to_owned())
}
