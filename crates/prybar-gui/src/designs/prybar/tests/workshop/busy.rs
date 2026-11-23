use std::time::Duration;

use gpui::TestAppContext;
use prybar_workshop::item::Point;

use super::super::window;
use super::page;
use crate::state::tests::fake_item::ASKED;
use crate::state::tools::History;

fn asked() -> usize {
    ASKED.with(std::cell::Cell::get)
}

/// Quick clicks ask only for the item that stays; a busy answer waits
/// the 30 s Steam asked for, then asks once more, never in a loop.
#[gpui::test]
fn busy_steam_is_asked_once_settled_and_after_its_wait(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    page(&state, 3, (1280., 800.), &mut cx);
    let before = asked();
    state.update(&mut cx, |s, cx| s.select_item(9, cx));
    state.update(&mut cx, |s, cx| s.select_item(8, cx));
    cx.executor().advance_clock(Duration::from_millis(600));
    cx.run_until_parked();
    assert_eq!(asked() - before, 1, "only the settled item");
    let h = state.read_with(&cx, |s, _| s.tools.item.history.get(&8).cloned());
    assert_eq!(
        h,
        Some(History::Busy {
            wait: 30,
            attempt: 1
        })
    );
    assert!(cx.debug_bounds("pb-hist-busy").is_some(), "calm message");
    cx.executor().advance_clock(Duration::from_secs(29));
    cx.run_until_parked();
    assert_eq!(asked() - before, 1, "waits");
    cx.executor().advance_clock(Duration::from_secs(2));
    cx.run_until_parked();
    assert_eq!(asked() - before, 2, "asks again after the wait");
    super::super::details::click(&mut cx, "pb-hist-retry");
    assert_eq!(asked() - before, 3, "Try again asks now");
}

/// No chart with fewer than three readings or all on one day.
#[gpui::test]
fn chart_needs_three_readings_over_two_days(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    page(&state, 1, (1280., 1000.), &mut cx);
    state.update(&mut cx, |s, cx| s.select_item(7, cx));
    let pt = |day| Point {
        day,
        subscriptions: 3,
        favorites: 1,
    };
    let cases = [
        (vec![], false),
        (vec![pt(1), pt(2)], false),
        (vec![pt(1), pt(1), pt(1)], false),
        (vec![pt(1), pt(2), pt(3)], true),
    ];
    for (series, shown) in cases {
        state.update(&mut cx, |s, cx| {
            s.tools.item.series = series.clone();
            cx.notify();
        });
        cx.run_until_parked();
        assert_eq!(cx.debug_bounds("pb-chart").is_some(), shown, "{series:?}");
        assert_eq!(cx.debug_bounds("pb-chart-none").is_some(), !shown);
    }
}
