use gpui::{ClipboardItem, Focusable as _, Modifiers, TestAppContext, VisualTestContext};

use super::{forms, open_sample, window};
use crate::actions::MOD;
use crate::state::Screen;

fn focus_field(cx: &mut VisualTestContext, selector: &'static str) {
    let b = cx.debug_bounds(selector).expect("field drawn");
    // The label sits on top; the box is the lower part of the bounds.
    let at = b.origin + gpui::point(b.size.width / 2., b.size.height * 0.75);
    cx.simulate_click(at, Modifiers::none());
    cx.run_until_parked();
}

/// Typing, shift-arrow selection, paste and undo in the publish title.
#[gpui::test]
fn title_edits(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Publish, cx));
    cx.run_until_parked();
    focus_field(&mut cx, "pb-pub-title");
    cx.simulate_input("Crate pack");
    let f = forms(&mut cx);
    let text = |cx: &mut VisualTestContext| f.title.read_with(cx, |i, _| i.text().to_owned());
    assert_eq!(text(&mut cx), "Crate pack");
    cx.simulate_keystrokes("shift-left shift-left shift-left shift-left");
    cx.write_to_clipboard(ClipboardItem::new_string("box\nset".into()));
    cx.simulate_keystrokes(&format!("{MOD}-v"));
    assert_eq!(
        text(&mut cx),
        "Crate box set",
        "line breaks pasted as spaces"
    );
    cx.simulate_keystrokes(&format!("{MOD}-z"));
    assert_eq!(text(&mut cx), "Crate pack");
    cx.simulate_keystrokes(&format!("{MOD}-a backspace"));
    assert_eq!(
        f.title.read_with(&cx, |i, _| i.error()),
        Some("Give the addon a title.")
    );
}

/// The search field filters the package live and escape clears it.
#[gpui::test]
fn search_types_and_clears(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    open_sample(&state, &mut cx);
    assert_eq!(state.read_with(&cx, |s, _| s.screen), Screen::Package);
    cx.simulate_keystrokes(&format!("{MOD}-f"));
    cx.simulate_input("mdl");
    assert_eq!(
        state.read_with(&cx, |s, _| s.search.clone()),
        Some("mdl".into())
    );
    cx.simulate_keystrokes("escape");
    assert_eq!(state.read_with(&cx, |s, _| s.search.clone()), None);
    let f = forms(&mut cx);
    assert_eq!(f.search.read_with(&cx, |i, _| i.text().to_owned()), "");
}

/// Enter in a one-line field submits; in the description it adds a line.
#[gpui::test]
fn enter_submits_or_breaks(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Publish, cx));
    cx.run_until_parked();
    let f = forms(&mut cx);
    let submitted = std::rc::Rc::new(std::cell::Cell::new(false));
    let seen = submitted.clone();
    let _sub = cx.update(|_, cx| {
        cx.subscribe(&f.title, move |_, ev, _| {
            seen.set(matches!(ev, crate::designs::input::InputEvent::Submit))
        })
    });
    focus_field(&mut cx, "pb-pub-title");
    cx.simulate_keystrokes("a enter");
    assert!(submitted.get());
    f.description.update(&mut cx, |i, cx| i.set_text("one", cx));
    cx.update(|window, cx| f.description.read(cx).focus_handle(cx).focus(window, cx));
    cx.simulate_keystrokes("enter t w o");
    assert_eq!(
        f.description.read_with(&cx, |i, _| i.text().to_owned()),
        "one\ntwo"
    );
}
