mod ime;
mod input;
mod publish;
mod shell;
mod tips;
mod tools;
mod views;

use gpui::{AppContext as _, Entity, Modifiers, TestAppContext, VisualTestContext};

use crate::app::{Forms, Root};
use crate::state::AppState;

pub(crate) fn window(
    cx: &mut TestAppContext,
    light: bool,
) -> (Entity<AppState>, VisualTestContext) {
    cx.update(|cx| cx.bind_keys(crate::actions::bindings()));
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |window, cx| {
            let state = cx.new(|_| {
                let mut s = AppState::default();
                s.light = light;
                s.tools.backend = crate::state::tests::fake::backend();
                s
            });
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

/// Every bar entry navigates, in both themes, and the view follows.
#[gpui::test]
fn nav_reaches_every_bar_screen(cx: &mut TestAppContext) {
    for light in [false, true] {
        let (state, mut cx) = window(cx, light);
        for label in [
            "model",
            "package",
            "decompile",
            "compile",
            "publish",
            "settings",
            "open",
        ] {
            let nav = cx
                .debug_bounds(format!("nav-{label}").leak())
                .expect("nav drawn");
            cx.simulate_mouse_move(nav.center(), None, Modifiers::none());
            cx.simulate_click(nav.center(), Modifiers::none());
            cx.run_until_parked();
            let screen = state.read_with(&cx, |s, _| s.screen);
            assert_eq!(screen.label().to_lowercase(), label);
            assert!(cx.debug_bounds("pb-view").is_some());
        }
    }
}

/// Opens the synthetic model, then the package, so the package ends up
/// selected and both kinds of screen have data.
pub(super) fn open_sample(state: &Entity<AppState>, cx: &mut VisualTestContext) {
    cx.executor().allow_parking();
    let dir = tempfile::tempdir().expect("tempdir").keep();
    let s = crate::sample::write(&dir).expect("sample written");
    state.update(cx, |st, cx| st.open(s.model, cx));
    cx.run_until_parked();
    state.update(cx, |st, cx| st.open(s.package, cx));
    cx.run_until_parked();
}

/// The text fields of the window's root view.
pub(super) fn forms(cx: &mut VisualTestContext) -> Forms {
    let root = cx
        .update(|window, _| window.root::<Root>())
        .flatten()
        .expect("root view");
    root.read_with(cx, |r, _| r.forms.clone())
}
