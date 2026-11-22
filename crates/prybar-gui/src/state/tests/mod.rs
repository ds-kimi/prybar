use std::path::PathBuf;

use gpui::{AppContext as _, Entity, TestAppContext};

mod ops;

use super::{AppState, Screen};

pub(super) fn open(cx: &mut TestAppContext, path: PathBuf) -> Entity<AppState> {
    // Real file reads run on the background executor.
    cx.executor().allow_parking();
    let state = cx.new(|_| AppState::default());
    state.update(cx, |s, cx| s.open(path, cx));
    cx.run_until_parked();
    state
}

#[gpui::test]
fn navigate_and_back(cx: &mut TestAppContext) {
    let state = cx.new(|_| AppState::default());
    state.update(cx, |s, cx| {
        s.navigate(Screen::Settings, cx);
        s.navigate(Screen::Compile, cx);
        s.back(cx);
        assert_eq!(s.screen, Screen::Settings);
        s.back(cx);
        s.back(cx);
        assert_eq!((s.screen, s.history.len()), (Screen::Home, 0));
    });
}

#[gpui::test]
fn theme_is_remembered(cx: &mut TestAppContext) {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("prybar").join("gui.txt");
    assert!(!super::prefs::load_light(&path), "dark by default");
    let state = cx.new(|_| AppState {
        prefs: Some(path.clone()),
        ..AppState::default()
    });
    state.update(cx, |s, cx| s.toggle_theme(cx));
    assert!(super::prefs::load_light(&path));
    state.update(cx, |s, cx| s.toggle_theme(cx));
    assert!(!super::prefs::load_light(&path));
}
