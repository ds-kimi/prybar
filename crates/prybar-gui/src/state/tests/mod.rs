use std::path::PathBuf;

use gpui::{AppContext as _, Entity, TestAppContext};

mod ops;

use super::{AppState, DesignKind, Screen};

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
        s.set_design(Some(DesignKind::Canvas), cx);
        s.navigate(Screen::Settings, cx);
        s.navigate(Screen::Compile, cx);
        s.back(cx);
        assert_eq!(s.screen, Screen::Settings);
        s.back(cx);
        s.back(cx);
        assert_eq!((s.screen, s.design), (Screen::Home, None));
    });
}
