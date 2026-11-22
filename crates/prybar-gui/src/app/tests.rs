use gpui::{
    AppContext as _, Entity, ExternalPaths, FileDropEvent, Modifiers, TestAppContext,
    VisualTestContext,
};

use super::Root;
use crate::actions::MOD;
use crate::state::{AppState, DesignKind, Screen};

fn window(cx: &mut TestAppContext) -> (Entity<AppState>, VisualTestContext) {
    cx.update(|cx| cx.bind_keys(crate::actions::bindings()));
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |window, cx| {
            let state = cx.new(|_| AppState::default());
            cx.new(|cx| Root::new(state, window, cx))
        })
        .expect("test window opens")
    });
    let mut vcx = VisualTestContext::from_window(window.into(), cx);
    let root = window.root(&mut vcx).expect("root view");
    let state = root.read_with(&vcx, |r, _| r.state.clone());
    vcx.run_until_parked();
    (state, vcx)
}

#[gpui::test]
fn card_click_opens_design(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx);
    for name in ["card-workbench", "card-canvas", "card-console"] {
        assert!(cx.debug_bounds(name).is_some(), "{name} drawn");
    }
    let card = cx.debug_bounds("card-canvas").expect("canvas card");
    cx.simulate_click(card.center(), Modifiers::none());
    assert_eq!(
        state.read_with(&cx, |s, _| s.design),
        Some(DesignKind::Canvas)
    );
    cx.run_until_parked();
    let nav = cx.debug_bounds("nav-settings").expect("nav drawn");
    cx.simulate_click(nav.center(), Modifiers::none());
    cx.run_until_parked();
    assert_eq!(state.read_with(&cx, |s, _| s.screen), Screen::Settings);
}

#[gpui::test]
fn keys_switch_design_and_back(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx);
    cx.simulate_keystrokes(&format!("{MOD}-3"));
    assert_eq!(
        state.read_with(&cx, |s, _| s.design),
        Some(DesignKind::Console)
    );
    cx.simulate_keystrokes(&format!("{MOD}-1"));
    assert_eq!(
        state.read_with(&cx, |s, _| s.design),
        Some(DesignKind::Workbench)
    );
    cx.simulate_keystrokes(&format!("{MOD}-f"));
    assert_eq!(
        state.read_with(&cx, |s, _| s.search.clone()),
        Some(String::new())
    );
    cx.simulate_keystrokes("escape");
    assert_eq!(state.read_with(&cx, |s, _| s.search.clone()), None);
    cx.simulate_keystrokes("escape");
    assert_eq!(state.read_with(&cx, |s, _| s.design), None);
    cx.simulate_keystrokes(&format!("{MOD}-2 {MOD}-0"));
    assert_eq!(state.read_with(&cx, |s, _| s.design), None);
}

#[gpui::test]
fn dropped_file_opens(cx: &mut TestAppContext) {
    cx.executor().allow_parking();
    let tmp = tempfile::tempdir().expect("tempdir");
    let sample = crate::sample::write(tmp.path()).expect("sample written");
    let (state, mut cx) = window(cx);
    let at = cx.debug_bounds("card-console").expect("card").center();
    let paths = ExternalPaths(vec![sample.model].into());
    cx.simulate_event(FileDropEvent::Entered {
        position: at,
        paths,
    });
    cx.simulate_event(FileDropEvent::Submit { position: at });
    cx.run_until_parked();
    state.read_with(&cx, |s, _| {
        assert_eq!(s.documents.len(), 1);
        assert_eq!(s.screen, Screen::Inspector);
    });
}
