use gpui::{Modifiers, TestAppContext};

use super::{forms, window};
use crate::state::Screen;

/// An addon folder in the content field shows its rejected files.
#[gpui::test]
fn addon_folder_is_checked_as_typed(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    let tmp = tempfile::tempdir().expect("tempdir");
    let dir = tmp.path();
    addon_dir(dir);
    super::details::new_popup(&state, &mut cx);
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

/// Search narrows the grid as typed, the empty result says what to do,
/// the list toggle sticks and New item opens an empty form.
#[gpui::test]
fn items_search_layout_and_new(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Publish, cx));
    cx.run_until_parked();
    assert!(cx.debug_bounds("pb-item-0").is_some(), "card drawn");
    let f = forms(&mut cx);
    let search = f.publish.items.search.clone();
    search.update(&mut cx, |i, cx| i.set_text("no such item", cx));
    cx.run_until_parked();
    assert!(cx.debug_bounds("pb-item-0").is_none(), "filtered out");
    assert!(cx.debug_bounds("pb-items-state").is_some(), "says why");
    search.update(&mut cx, |i, cx| i.set_text("CRATE", cx));
    cx.run_until_parked();
    let b = cx.debug_bounds("pb-items-list").expect("toggle drawn");
    cx.simulate_click(b.center(), Modifiers::none());
    cx.run_until_parked();
    state.read_with(&cx, |s, _| assert!(!s.tools.steam.catalog.grid));
    super::details::click(&mut cx, "pb-item-0");
    super::details::click(&mut cx, "pb-items-new");
    assert!(cx.debug_bounds("pb-pub-popup").is_some(), "popup open");
    let id = f.publish.item_id.read_with(&cx, |i, _| i.text().to_owned());
    assert_eq!(id, "", "new item has no id");
}

/// A tool addon with one Lua file and one file the game refuses.
pub(super) fn addon_dir(dir: &std::path::Path) {
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
