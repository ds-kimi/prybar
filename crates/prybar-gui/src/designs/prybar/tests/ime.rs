use gpui::{EntityInputHandler as _, Focusable as _, TestAppContext, VisualTestContext};

use super::{forms, window};
use crate::designs::input::TextInput;
use crate::state::Screen;

fn title(cx: &mut TestAppContext) -> (gpui::Entity<TextInput>, VisualTestContext) {
    let (state, mut cx) = window(cx, false);
    let cx = &mut cx;
    state.update(cx, |s, cx| s.navigate(Screen::Publish, cx));
    cx.run_until_parked();
    let t = forms(cx).publish.title;
    cx.update(|window, cx| t.read(cx).focus_handle(cx).focus(window, cx));
    cx.run_until_parked();
    (t, cx.clone())
}

/// A dead key marks its accent, the next key replaces it with the
/// composed char and commits; text around it is untouched.
#[gpui::test]
fn marked_text_replaces_then_commits(cx: &mut TestAppContext) {
    let (t, mut cx) = title(cx);
    let cx = &mut cx;
    cx.simulate_input("caf");
    cx.update(|w, cx| {
        t.update(cx, |i, cx| {
            i.replace_and_mark_text_in_range(None, "´", None, w, cx);
            assert_eq!(i.marked_text_range(w, cx), Some(3..4));
            i.replace_text_in_range(None, "é", w, cx);
            assert_eq!(i.marked_text_range(w, cx), None);
        })
    });
    cx.simulate_input("!");
    assert_eq!(t.read_with(cx, |i, _| i.text().to_owned()), "café!");
}

/// An IME composing several chars keeps the caret where it asks, and
/// counts in UTF-16 so emoji before it do not shift the range.
#[gpui::test]
fn composition_counts_utf16(cx: &mut TestAppContext) {
    let (t, mut cx) = title(cx);
    let cx = &mut cx;
    cx.simulate_input("😀");
    cx.update(|w, cx| {
        t.update(cx, |i, cx| {
            i.replace_and_mark_text_in_range(None, "にほ", Some(1..1), w, cx);
            assert_eq!(i.marked_text_range(w, cx), Some(2..4));
            let sel = i.selected_text_range(false, w, cx).map(|s| s.range);
            assert_eq!(sel, Some(3..3));
            i.replace_text_in_range(None, "日本", w, cx);
        })
    });
    assert_eq!(t.read_with(cx, |i, _| i.text().to_owned()), "😀日本");
}

/// Long one-line text slides so the caret stays inside the field.
#[gpui::test]
fn long_text_keeps_caret_visible(cx: &mut TestAppContext) {
    let (t, mut cx) = title(cx);
    let cx = &mut cx;
    let field = cx.debug_bounds("pb-pub-title").expect("field drawn");
    cx.simulate_input(&"wide crate ".repeat(30));
    cx.run_until_parked();
    let caret = cx.update(|w, cx| {
        t.update(cx, |i, cx| {
            let end = i.text_length_utf16(w, cx).unwrap_or(0);
            i.bounds_for_range(end..end, field, w, cx)
        })
    });
    let caret = caret.expect("caret bounds");
    assert!(caret.origin.x <= field.right(), "caret at {caret:?}");
    assert!(caret.origin.x >= field.left());
}
