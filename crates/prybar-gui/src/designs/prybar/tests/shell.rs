use gpui::{Modifiers, TestAppContext};

use super::window;
use crate::actions::MOD;
use crate::state::{Job, JobStatus, Screen, ToastKind};

#[gpui::test]
fn toggle_switches_theme(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    let toggle = cx.debug_bounds("pb-theme").expect("theme toggle");
    cx.simulate_click(toggle.center(), Modifiers::none());
    assert!(state.read_with(&cx, |s, _| s.light));
    cx.simulate_keystrokes(&format!("{MOD}-shift-l"));
    assert!(!state.read_with(&cx, |s, _| s.light));
}

/// Row 8 is "Go to Textures", which only the palette reaches.
#[gpui::test]
fn palette_reaches_hidden_screens(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, true);
    cx.simulate_keystrokes(&format!("{MOD}-k"));
    assert!(cx.debug_bounds("pb-palette").is_some());
    let row = cx.debug_bounds("cmd-8").expect("palette row");
    cx.simulate_click(row.center(), Modifiers::none());
    cx.run_until_parked();
    state.read_with(&cx, |s, _| {
        assert!(!s.palette);
        assert_eq!(s.screen, Screen::TexturePreview);
    });
    cx.simulate_keystrokes(&format!("{MOD}-k escape"));
    assert!(!state.read_with(&cx, |s, _| s.palette));
}

#[gpui::test]
fn status_and_toasts_show(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    assert!(cx.debug_bounds("pb-job").is_none());
    state.update(&mut cx, |s, cx| {
        s.jobs.push(Job {
            id: 99,
            label: "Decompiling crate.mdl".into(),
            status: JobStatus::Running,
        });
        s.toast(ToastKind::Error, "Bad header".into(), cx);
    });
    cx.run_until_parked();
    assert!(cx.debug_bounds("pb-job").is_some());
    assert!(cx.debug_bounds("pb-toasts").is_some());
}

/// The logo is the way back Home from any screen.
#[gpui::test]
fn logo_goes_home(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    let settings = cx.debug_bounds("nav-settings").expect("settings entry");
    cx.simulate_click(settings.center(), Modifiers::none());
    assert_eq!(state.read_with(&cx, |s, _| s.screen), Screen::Settings);
    let logo = cx.debug_bounds("pb-logo").expect("logo");
    cx.simulate_click(logo.center(), Modifiers::none());
    assert_eq!(state.read_with(&cx, |s, _| s.screen), Screen::Home);
}
