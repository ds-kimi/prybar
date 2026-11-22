use gpui::{AppContext as _, Entity, Modifiers, TestAppContext, VisualTestContext};

use crate::actions::MOD;
use crate::app::Root;
use crate::state::{AppState, DesignKind, Screen};

/// A Console window; with `sample`, the synthetic model and package are
/// open and the package is selected.
fn window(cx: &mut TestAppContext, sample: bool) -> (Entity<AppState>, VisualTestContext) {
    cx.executor().allow_parking();
    cx.update(|cx| cx.bind_keys(crate::actions::bindings()));
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |window, cx| {
            let state = cx.new(|_| {
                let mut s = AppState::default();
                s.design = Some(DesignKind::Console);
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
                .expect("crumb drawn");
            cx.simulate_mouse_move(nav.center(), None, Modifiers::none());
            cx.simulate_click(nav.center(), Modifiers::none());
            cx.run_until_parked();
            assert_eq!(state.read_with(&cx, |s, _| s.screen), screen);
            for part in ["cs-view", "cs-bar", "cs-foot"] {
                assert!(cx.debug_bounds(part).is_some(), "{part} on {label}");
            }
        }
    }
}

#[gpui::test]
fn typing_runs_commands(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    cx.simulate_input("go settings");
    assert!(cx.debug_bounds("cs-list").is_some(), "typing opens the bar");
    cx.simulate_keystrokes("enter");
    state.read_with(&cx, |s, _| {
        assert_eq!(s.screen, Screen::Settings);
        assert_eq!(s.search, None);
    });
    cx.simulate_keystrokes(&format!("{MOD}-k down down up"));
    assert_eq!(state.read_with(&cx, |s, _| s.cursor), 1);
    cx.simulate_keystrokes("enter");
    assert_eq!(state.read_with(&cx, |s, _| s.screen), Screen::Inspector);
    cx.simulate_keystrokes(&format!("{MOD}-k"));
    let row = cx.debug_bounds("cs-cmd-3").expect("command row");
    cx.simulate_mouse_move(row.center(), None, Modifiers::none());
    cx.simulate_click(row.center(), Modifiers::none());
    assert_eq!(state.read_with(&cx, |s, _| s.screen), Screen::Decompile);
    cx.simulate_input("zzzq");
    cx.simulate_keystrokes("enter escape");
    assert_eq!(state.read_with(&cx, |s, _| s.screen), Screen::Decompile);
}

#[gpui::test]
fn files_and_decompile(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, true);
    state.update(&mut cx, |s, cx| s.navigate(Screen::Home, cx));
    cx.run_until_parked();
    let file = cx.debug_bounds("cs-file-0").expect("model row");
    cx.simulate_click(file.center(), Modifiers::none());
    state.read_with(&cx, |s, _| {
        assert_eq!(s.selected, Some(0));
        assert_eq!(s.screen, Screen::Inspector);
    });
    cx.simulate_input("decompile");
    cx.simulate_keystrokes("enter");
    cx.run_until_parked();
    let report = state.read_with(&cx, |s, _| s.last_decompile.clone());
    let report = report.expect("decompile ran");
    assert_eq!(report.models.len(), 1, "{:?}", report.failures);
    cx.simulate_input("/no/such/file.mdl");
    cx.simulate_keystrokes("enter");
    cx.run_until_parked();
    let last = state.read_with(&cx, |s, _| s.toasts.last().map(|t| t.kind));
    assert_eq!(last, Some(crate::state::ToastKind::Error));
}
