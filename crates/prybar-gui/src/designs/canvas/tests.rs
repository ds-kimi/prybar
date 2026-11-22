use gpui::{AppContext as _, Entity, Modifiers, TestAppContext, VisualTestContext};

use crate::actions::MOD;
use crate::app::Root;
use crate::state::{AppState, DesignKind, Screen};

/// A Canvas window; with `sample`, the synthetic model and package are
/// open and the package is selected.
fn window(cx: &mut TestAppContext, sample: bool) -> (Entity<AppState>, VisualTestContext) {
    cx.executor().allow_parking();
    cx.update(|cx| cx.bind_keys(crate::actions::bindings()));
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |window, cx| {
            let state = cx.new(|_| {
                let mut s = AppState::default();
                s.design = Some(DesignKind::Canvas);
                s
            });
            cx.new(|cx| Root::new(state, window, cx))
        })
        .expect("test window opens")
    });
    let mut vcx = VisualTestContext::from_window(window.into(), cx);
    let root = window.root(&mut vcx).expect("root view");
    let state = root.read_with(&vcx, |r, _| r.state.clone());
    if sample {
        let dir = tempfile::tempdir().expect("tempdir").keep();
        let s = crate::sample::write(&dir).expect("sample written");
        state.update(&mut vcx, |st, cx| st.open(s.model, cx));
        vcx.run_until_parked();
        state.update(&mut vcx, |st, cx| st.open(s.package, cx));
    }
    vcx.run_until_parked();
    (state, vcx)
}

#[gpui::test]
fn every_screen_renders(cx: &mut TestAppContext) {
    for sample in [false, true] {
        let (state, mut cx) = window(cx, sample);
        for screen in Screen::ALL {
            let label = screen.label().to_lowercase();
            let nav = cx
                .debug_bounds(format!("nav-{label}").leak())
                .expect("tab drawn");
            cx.simulate_mouse_move(nav.center(), None, Modifiers::none());
            cx.simulate_click(nav.center(), Modifiers::none());
            cx.run_until_parked();
            assert_eq!(state.read_with(&cx, |s, _| s.screen), screen);
            assert!(cx.debug_bounds("cv-view").is_some(), "{label} drawn");
            assert!(cx.debug_bounds("cv-steps").is_some());
        }
    }
}

#[gpui::test]
fn file_cards_and_games(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, true);
    assert!(cx.debug_bounds("cv-drop").is_none());
    let card = cx.debug_bounds("cv-file-0").expect("model card");
    cx.simulate_mouse_move(card.center(), None, Modifiers::none());
    cx.simulate_click(card.center(), Modifiers::none());
    assert_eq!(state.read_with(&cx, |s, _| s.selected), Some(0));
    state.update(&mut cx, |s, cx| s.navigate(Screen::Compile, cx));
    cx.run_until_parked();
    let game = cx.debug_bounds("cv-game-2").expect("game card");
    cx.simulate_click(game.center(), Modifiers::none());
    assert_eq!(state.read_with(&cx, |s, _| s.game), 2);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Home, cx));
    cx.run_until_parked();
    assert!(cx.debug_bounds("cv-drop").is_some());
}

#[gpui::test]
fn search_and_decompile(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, true);
    assert_eq!(state.read_with(&cx, |s, _| s.screen), Screen::Package);
    cx.simulate_keystrokes(&format!("{MOD}-f"));
    cx.simulate_input("mdl");
    assert_eq!(
        state.read_with(&cx, |s, _| s.search.clone()),
        Some("mdl".into())
    );
    cx.simulate_keystrokes("escape");
    let card = cx.debug_bounds("cv-file-0").expect("model card");
    cx.simulate_click(card.center(), Modifiers::none());
    state.update(&mut cx, |s, cx| s.navigate(Screen::Decompile, cx));
    cx.run_until_parked();
    let split = cx.debug_bounds("cv-split").expect("split chip");
    cx.simulate_click(split.center(), Modifiers::none());
    assert!(state.read_with(&cx, |s, _| s.split_qc));
    let run = cx.debug_bounds("cv-decompile").expect("run pill");
    cx.simulate_click(run.center(), Modifiers::none());
    cx.run_until_parked();
    let report = state.read_with(&cx, |s, _| s.last_decompile.clone());
    let report = report.expect("decompile ran");
    assert_eq!(report.models.len(), 1, "{:?}", report.failures);
}
