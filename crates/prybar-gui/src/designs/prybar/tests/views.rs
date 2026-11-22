use gpui::{Modifiers, TestAppContext, VisualTestContext};

use super::{open_sample, window};
use crate::state::{AppState, Screen};

fn click(cx: &mut VisualTestContext, selector: &'static str) {
    let b = cx
        .debug_bounds(selector)
        .unwrap_or_else(|| panic!("{selector} drawn"));
    cx.simulate_mouse_move(b.center(), None, Modifiers::none());
    cx.simulate_click(b.center(), Modifiers::none());
    cx.run_until_parked();
}

/// Every screen draws with and without files, in both themes.
#[gpui::test]
fn every_screen_renders(cx: &mut TestAppContext) {
    for (light, sample) in [(false, false), (true, false), (false, true), (true, true)] {
        let (state, mut cx) = window(cx, light);
        if sample {
            open_sample(&state, &mut cx);
        }
        for screen in Screen::ALL {
            state.update(&mut cx, |s, cx| s.navigate(screen, cx));
            cx.run_until_parked();
            assert!(cx.debug_bounds("pb-view").is_some(), "{screen:?}");
            let needs_file = matches!(screen, Screen::Inspector | Screen::Package);
            assert_eq!(
                cx.debug_bounds("pb-empty-open").is_some(),
                needs_file && !sample
            );
        }
        if sample {
            state.update(&mut cx, |s, cx| s.navigate(Screen::Inspector, cx));
            cx.run_until_parked();
            assert!(cx.debug_bounds("pb-facts").is_some());
        }
    }
}

/// Empty model screen leads to Open; model header leads to the previews.
#[gpui::test]
fn model_links_and_empty_open(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Inspector, cx));
    cx.run_until_parked();
    click(&mut cx, "pb-empty-open");
    assert_eq!(state.read_with(&cx, |s, _| s.screen), Screen::Home);
    open_sample(&state, &mut cx);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Inspector, cx));
    cx.run_until_parked();
    click(&mut cx, "pb-to-tex");
    assert_eq!(
        state.read_with(&cx, |s, _| s.screen),
        Screen::TexturePreview
    );
    cx.run_until_parked();
    assert!(cx.debug_bounds("pb-tex-preview").is_some());
    assert!(
        cx.debug_bounds("pb-tex-0").is_some(),
        "sample material listed"
    );
}

/// Recent row reopens, decompile runs and reports, the option and the
/// game pick toggle.
#[gpui::test]
fn main_click_paths(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    open_sample(&state, &mut cx);
    let docs = |c: &VisualTestContext| state.read_with(c, |s: &AppState, _| s.documents.len());
    state.update(&mut cx, |s, cx| s.navigate(Screen::Home, cx));
    cx.run_until_parked();
    click(&mut cx, "pb-recent-0");
    assert_eq!(docs(&cx), 3);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Decompile, cx));
    cx.run_until_parked();
    click(&mut cx, "pb-split");
    click(&mut cx, "pb-decompile");
    assert!(state.read_with(&cx, |s, _| s.split_qc && s.last_decompile.is_some()));
    assert!(cx.debug_bounds("pb-result").is_some());
    state.update(&mut cx, |s, cx| s.navigate(Screen::Compile, cx));
    cx.run_until_parked();
    click(&mut cx, "pb-game-1");
    assert_eq!(state.read_with(&cx, |s, _| s.tools.game), 1);
}
