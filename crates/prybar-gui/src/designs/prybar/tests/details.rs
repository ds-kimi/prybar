//! The details panel and the popups on top of the item grid.

use gpui::{Entity, Modifiers, TestAppContext, VisualTestContext};

use super::{forms, window};
use crate::state::{AppState, Screen};

/// Publish screen, then New item: the form lives in that popup. The
/// window is tall so the whole popup fits without scrolling.
pub(crate) fn new_popup(state: &Entity<AppState>, cx: &mut VisualTestContext) {
    cx.simulate_resize(gpui::size(gpui::px(1280.), gpui::px(1600.)));
    state.update(cx, |s, cx| s.navigate(crate::state::Screen::Publish, cx));
    cx.run_until_parked();
    let b = cx.debug_bounds("pb-items-new").expect("new drawn");
    cx.simulate_click(b.center(), Modifiers::none());
    cx.run_until_parked();
}

/// Steam is asked at start; a click on an item shows its stats, chart
/// and history, Edit opens the popup on it, the badge checks again.
#[gpui::test]
fn item_click_shows_details_and_edit_fills(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    cx.simulate_resize(gpui::size(gpui::px(1280.), gpui::px(1600.)));
    state.update(&mut cx, |s, cx| s.navigate(Screen::Publish, cx));
    cx.run_until_parked();
    click(&mut cx, "pb-item-0");
    cx.executor()
        .advance_clock(std::time::Duration::from_millis(600));
    cx.run_until_parked();
    assert!(cx.debug_bounds("pb-details").is_some(), "details drawn");
    assert!(cx.debug_bounds("pb-history").is_some(), "history drawn");
    state.read_with(&cx, |s, _| {
        let h = s.tools.item.history.get(&7);
        assert!(matches!(h, Some(crate::state::tools::History::Ready(v)) if v.changes.len() == 1));
    });
    click(&mut cx, "pb-det-edit");
    assert!(cx.debug_bounds("pb-pub-popup").is_some(), "popup open");
    let f = forms(&mut cx);
    let id = f.publish.item_id.read_with(&cx, |i, _| i.text().to_owned());
    assert_eq!(id, "7");
    cx.simulate_keystrokes("escape");
    cx.run_until_parked();
    click(&mut cx, "pb-steam");
    state.read_with(&cx, |s, _| assert!(s.tools.steam.status.is_some()));
}

/// Content sent from a folder in the edit popup links it to the item.
#[gpui::test]
fn edit_links_the_folder(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    let tmp = tempfile::tempdir().expect("tempdir");
    super::publish::addon_dir(tmp.path());
    cx.simulate_resize(gpui::size(gpui::px(1280.), gpui::px(1600.)));
    let item = Some(crate::state::tests::fake::item());
    let form = cx.update(|w, cx| {
        let root = w.root::<crate::app::Root>().flatten().expect("root");
        root.update(cx, |r, cx| r.open_publish(item.clone(), w, cx));
        root.read(cx).forms.publish.clone()
    });
    form.title
        .update(&mut cx, |i, cx| i.set_text("Crate pack 2", cx));
    let dir = tmp.path().display().to_string();
    form.content.update(&mut cx, |i, cx| i.set_text(&dir, cx));
    cx.run_until_parked();
    click(&mut cx, "pb-pub-go");
    state.read_with(&cx, |s, _| {
        assert_eq!(
            s.tools.item.links.0.get(&7),
            Some(&tmp.path().to_path_buf())
        );
    });
}

pub(super) fn click(cx: &mut gpui::VisualTestContext, selector: &'static str) {
    let b = cx
        .debug_bounds(selector)
        .unwrap_or_else(|| panic!("{selector} drawn"));
    cx.simulate_click(b.center(), Modifiers::none());
    cx.run_until_parked();
}
