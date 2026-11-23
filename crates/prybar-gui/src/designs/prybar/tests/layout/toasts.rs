use gpui::{Modifiers, TestAppContext, VisualTestContext};

use super::super::window;
use crate::state::toast::{ENTER, LEAVE, LIFE, MAX_VISIBLE};
use crate::state::{AppState, ToastKind};

fn say(state: &gpui::Entity<AppState>, cx: &mut VisualTestContext, kind: ToastKind, n: u64) {
    state.update(cx, |s, cx| s.toast(kind, format!("Toast {n}"), cx));
    cx.executor().advance_clock(ENTER);
    // Animations read the wall clock, not the test clock, so the slide
    // is let finish for real before anything is hit tested.
    std::thread::sleep(ENTER);
    cx.update(|w, _| w.refresh());
    cx.run_until_parked();
}

fn ids(state: &gpui::Entity<AppState>, cx: &VisualTestContext) -> Vec<u64> {
    state.read_with(cx, |s, _| s.toasts.iter().map(|t| t.id).collect())
}

/// Newest sits lowest; a confirmation leaves after its life plus the
/// exit; an error stays until clicked; past the cap the oldest leaves.
#[gpui::test]
fn toasts_stack_leave_and_cap(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    say(&state, &mut cx, ToastKind::Error, 1);
    say(&state, &mut cx, ToastKind::Info, 2);
    let (a, b) = (cx.debug_bounds("pb-toast-1"), cx.debug_bounds("pb-toast-2"));
    assert!(
        a.expect("first").top() < b.expect("second").top(),
        "newest at the bottom"
    );
    cx.executor().advance_clock(LIFE - ENTER);
    cx.run_until_parked();
    assert!(
        state.read_with(&cx, |s, _| s.toasts[1].leaving),
        "exit playing"
    );
    cx.executor().advance_clock(LEAVE);
    cx.run_until_parked();
    assert_eq!(ids(&state, &cx), vec![1], "info gone, error stays");
    let b = cx.debug_bounds("pb-toast-1").expect("error drawn");
    cx.simulate_click(b.center(), Modifiers::none());
    cx.executor().advance_clock(LEAVE);
    cx.run_until_parked();
    assert!(ids(&state, &cx).is_empty(), "click dismisses");
    for n in 0..=MAX_VISIBLE as u64 {
        say(&state, &mut cx, ToastKind::Error, n);
    }
    cx.executor().advance_clock(LEAVE);
    cx.run_until_parked();
    assert_eq!(ids(&state, &cx).len(), MAX_VISIBLE, "oldest left");
}

/// The pointer on a confirmation holds it; leaving gives it a new life.
#[gpui::test]
fn hover_pauses_the_timer(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    say(&state, &mut cx, ToastKind::Info, 1);
    let b = cx.debug_bounds("pb-toast-1").expect("toast drawn");
    cx.simulate_mouse_move(b.center(), None, Modifiers::none());
    cx.run_until_parked();
    assert!(
        state.read_with(&cx, |s, _| s.toasts[0].hovered),
        "hover seen"
    );
    cx.executor().advance_clock(LIFE * 2);
    cx.run_until_parked();
    assert_eq!(ids(&state, &cx), vec![1], "held while hovered");
    cx.simulate_mouse_move(
        gpui::point(gpui::px(5.), gpui::px(5.)),
        None,
        Modifiers::none(),
    );
    cx.executor().advance_clock(LIFE + LEAVE);
    cx.run_until_parked();
    assert!(ids(&state, &cx).is_empty(), "gone after a fresh life");
}
