use gpui::{Modifiers, TestAppContext};

use super::{forms, window};
use crate::state::Screen;

/// Opening the screen asks Steam once; a listed item fills the form.
#[gpui::test]
fn steam_line_and_items_fill_the_form(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Publish, cx));
    cx.run_until_parked();
    assert!(cx.debug_bounds("pb-pub-steam").is_some());
    let b = cx.debug_bounds("pb-pub-item-0").expect("item row drawn");
    cx.simulate_click(b.center(), Modifiers::none());
    cx.run_until_parked();
    let f = forms(&mut cx);
    let id = f.publish.item_id.read_with(&cx, |i, _| i.text().to_owned());
    assert_eq!(id, "7");
    let b = cx.debug_bounds("pb-pub-retry").expect("retry drawn");
    cx.simulate_click(b.center(), Modifiers::none());
    cx.run_until_parked();
    state.read_with(&cx, |s, _| assert!(s.tools.steam.status.is_some()));
}

/// An addon folder in the content field shows its rejected files.
#[gpui::test]
fn addon_folder_is_checked_as_typed(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    let tmp = tempfile::tempdir().expect("tempdir");
    let dir = tmp.path();
    addon_dir(dir);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Publish, cx));
    let f = forms(&mut cx);
    let text = dir.display().to_string();
    f.publish
        .content
        .update(&mut cx, |i, cx| i.set_text(&text, cx));
    cx.run_until_parked();
    let first = state.read_with(&cx, |s, _| {
        let found = s.tools.steam.addon.as_ref().map(|a| a.1.clone());
        found
            .and_then(Result::ok)
            .map(|c| c.rejected[0].path.clone())
    });
    assert_eq!(first.as_deref(), Some("evil.exe"));
    assert!(cx.debug_bounds("pb-pub-addon").is_some(), "panel drawn");
    f.publish
        .content
        .update(&mut cx, |i, cx| i.set_text("", cx));
    cx.run_until_parked();
    state.read_with(&cx, |s, _| assert!(s.tools.steam.addon.is_none()));
}

/// A tool addon with one Lua file and one file the game refuses.
fn addon_dir(dir: &std::path::Path) {
    for (rel, body) in [
        ("lua/a.lua", "x"),
        ("evil.exe", "x"),
        (
            "addon.json",
            r#"{"title":"A","type":"tool","tags":["fun"]}"#,
        ),
    ] {
        std::fs::create_dir_all(dir.join(rel).parent().expect("parent")).expect("dirs");
        std::fs::write(dir.join(rel), body).expect("written");
    }
}
