use gpui::{AppContext as _, Entity, Modifiers, TestAppContext, VisualTestContext};

use crate::actions::MOD;
use crate::app::Root;
use crate::state::{AppState, DesignKind, Screen};

/// A Workbench window; with `sample`, the synthetic model and package are
/// open and the package is selected.
fn window(cx: &mut TestAppContext, sample: bool) -> (Entity<AppState>, VisualTestContext) {
    cx.executor().allow_parking();
    cx.update(|cx| cx.bind_keys(crate::actions::bindings()));
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |window, cx| {
            let state = cx.new(|_| {
                let mut s = AppState::default();
                s.design = Some(DesignKind::Workbench);
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
        // One at a time, so the package is always the last one selected.
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
                .expect("nav drawn");
            cx.simulate_mouse_move(nav.center(), None, Modifiers::none());
            cx.simulate_click(nav.center(), Modifiers::none());
            cx.run_until_parked();
            assert_eq!(state.read_with(&cx, |s, _| s.screen), screen);
            assert!(cx.debug_bounds("wb-view").is_some(), "{label} drawn");
            assert!(cx.debug_bounds("wb-inspector").is_some());
            assert!(cx.debug_bounds("wb-status").is_some());
        }
    }
}

#[gpui::test]
fn palette_runs_commands(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    cx.simulate_keystrokes(&format!("{MOD}-k"));
    assert!(state.read_with(&cx, |s, _| s.palette));
    // Row 6 is "Go to Settings", in Screen::ALL order.
    let row = cx.debug_bounds("cmd-6").expect("palette row");
    cx.simulate_click(row.center(), Modifiers::none());
    cx.run_until_parked();
    state.read_with(&cx, |s, _| {
        assert!(!s.palette);
        assert_eq!(s.screen, Screen::Settings);
    });
    let light = cx.debug_bounds("wb-light").expect("theme toggle");
    cx.simulate_click(light.center(), Modifiers::none());
    assert!(state.read_with(&cx, |s, _| s.light));
    cx.simulate_keystrokes(&format!("{MOD}-k escape"));
    assert!(!state.read_with(&cx, |s, _| s.palette));
    assert_eq!(state.read_with(&cx, |s, _| s.screen), Screen::Settings);
}

#[gpui::test]
fn tabs_search_and_decompile(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, true);
    assert_eq!(state.read_with(&cx, |s, _| s.screen), Screen::Package);
    cx.simulate_keystrokes(&format!("{MOD}-f"));
    cx.simulate_input("mdl");
    cx.simulate_keystrokes("backspace");
    assert_eq!(
        state.read_with(&cx, |s, _| s.search.clone()),
        Some("md".into())
    );
    let tab = cx.debug_bounds("tab-0").expect("model tab");
    cx.simulate_click(tab.center(), Modifiers::none());
    assert_eq!(state.read_with(&cx, |s, _| s.selected), Some(0));
    state.update(&mut cx, |s, cx| s.navigate(Screen::Decompile, cx));
    cx.run_until_parked();
    let split = cx.debug_bounds("wb-split").expect("split toggle");
    cx.simulate_click(split.center(), Modifiers::none());
    assert!(state.read_with(&cx, |s, _| s.split_qc));
    let run = cx.debug_bounds("wb-decompile").expect("run button");
    cx.simulate_click(run.center(), Modifiers::none());
    cx.run_until_parked();
    let report = state.read_with(&cx, |s, _| s.last_decompile.clone());
    let report = report.expect("decompile ran");
    assert_eq!(report.models.len(), 1, "{:?}", report.failures);
}
