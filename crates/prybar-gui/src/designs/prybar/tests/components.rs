//! The shared select, popup and Steam badge.

use gpui::{Modifiers, TestAppContext, VisualTestContext};
use prybar_workshop::SteamStatus;

use super::{forms, window};
use crate::app::Root;
use crate::designs::modal::{Modal, EXIT};
use crate::state::Screen;

/// Arrows, Enter, Escape and type-to-jump on the visibility select.
#[gpui::test]
fn select_follows_the_keyboard(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Publish, cx));
    cx.run_until_parked();
    let vis = forms(&mut cx).publish.visibility;
    let current = |cx: &mut VisualTestContext| vis.read_with(cx, |v, _| v.current());
    assert_eq!(current(&mut cx), 2, "new items start hidden");
    let b = cx.debug_bounds("pb-pub-visibility").expect("select drawn");
    cx.simulate_click(b.center(), Modifiers::none());
    assert!(
        cx.debug_bounds("pb-pub-visibility-opt-0").is_some(),
        "list open"
    );
    cx.simulate_keystrokes("up up enter");
    assert_eq!(current(&mut cx), 0);
    assert!(
        cx.debug_bounds("pb-pub-visibility-opt-0").is_none(),
        "closed"
    );
    cx.simulate_keystrokes("u");
    assert_eq!(current(&mut cx), 3, "u jumps to Unlisted");
    cx.simulate_keystrokes("down down escape");
    assert_eq!(current(&mut cx), 3, "escape keeps the choice");
    let form = forms(&mut cx).publish;
    let picked = cx.update(|_, cx| form.form(cx).visibility);
    assert_eq!(picked, "unlisted");
}

fn open(cx: &mut VisualTestContext, dirty: bool) -> gpui::Entity<Modal> {
    let root = cx.update(|w, _| w.root::<Root>().flatten()).expect("root");
    // The output field is not on the home screen, so it is free to host.
    let input = forms(cx).out;
    cx.update(|w, cx| {
        let body = input.clone().into();
        let modal = Modal::new("Edit", body, move |_| dirty, cx);
        root.update(cx, |r, cx| r.show_modal(modal.clone(), w, cx));
        modal
    })
}

/// Escape closes a clean popup; a dirty one asks first; Tab stays in.
#[gpui::test]
fn modal_traps_focus_and_confirms(cx: &mut TestAppContext) {
    let (_state, mut cx) = window(cx, false);
    let modal = open(&mut cx, false);
    cx.run_until_parked();
    let inside = |cx: &mut VisualTestContext| {
        cx.update(|w, cx| modal.read(cx).focus_handle().contains_focused(w, cx))
    };
    for _ in 0..6 {
        cx.simulate_keystrokes("tab");
        assert!(inside(&mut cx), "tab stays in the popup");
    }
    cx.simulate_keystrokes("escape");
    cx.executor().advance_clock(EXIT);
    cx.run_until_parked();
    assert!(cx.debug_bounds("pb-modal").is_none(), "clean popup closes");
    open(&mut cx, true);
    cx.simulate_keystrokes("escape");
    assert!(cx.debug_bounds("pb-modal-confirm").is_some(), "dirty asks");
    let b = cx.debug_bounds("pb-modal-keep").expect("keep drawn");
    cx.simulate_click(b.center(), Modifiers::none());
    assert!(cx.debug_bounds("pb-modal-confirm").is_none());
    cx.simulate_click(gpui::point(gpui::px(2.), gpui::px(2.)), Modifiers::none());
    let b = cx
        .debug_bounds("pb-modal-discard")
        .expect("outside click asks too");
    cx.simulate_click(b.center(), Modifiers::none());
    cx.executor().advance_clock(EXIT);
    cx.run_until_parked();
    assert!(cx.debug_bounds("pb-modal").is_none(), "discard closes");
}

/// Ready shows the name and a picture; any other state the plain label.
#[gpui::test]
fn badge_follows_steam(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    let (status, avatar) = state.read_with(&cx, |s, _| {
        (s.tools.steam.status.clone(), s.tools.steam.avatar.is_some())
    });
    assert!(matches!(status, Some(SteamStatus::Ready { .. })));
    assert!(avatar, "picture read once Steam is ready");
    assert!(cx.debug_bounds("pb-steam-dot").is_some());
    let words = state.read_with(&cx, |s, _| s.tools.steam.sentence());
    assert_eq!(words.1, Some(true));
    for (bad, hint) in [
        (SteamStatus::NotRunning, "Start Steam"),
        (SteamStatus::NotLoggedIn, "Log in"),
        (SteamStatus::NotOwned, "does not own"),
        (SteamStatus::Disabled, "no Steam Workshop support"),
    ] {
        state.update(&mut cx, |s, _| s.tools.steam.status = Some(bad));
        let (why, ok) = state.read_with(&cx, |s, _| s.tools.steam.sentence());
        assert_eq!(ok, Some(false));
        assert!(why.contains(hint), "{why}");
    }
}
