use gpui::{point, px, Modifiers, ScrollDelta, ScrollEvent, TestAppContext};

use super::super::window;
use super::page;

/// Every card drawn has the same size at small, default and large
/// windows, with or without the details panel, and the row is centred.
#[gpui::test]
fn cards_are_equal_at_every_width(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    for size in [(900., 600.), (1280., 800.), (1920., 1200.)] {
        for picked in [false, true] {
            page(&state, 200, size, &mut cx);
            if picked {
                state.update(&mut cx, |s, cx| s.select_item(7, cx));
                cx.run_until_parked();
                cx.update(|w, _| w.refresh());
                cx.run_until_parked();
            }
            check(&mut cx, format!("{size:?} picked {picked}"));
            if picked {
                state.update(&mut cx, |s, cx| s.select_item(7, cx));
            }
        }
    }
}

/// Cards on screen share one size; the row is centred in the grid,
/// which stays inside the page.
fn check(cx: &mut gpui::VisualTestContext, label: String) {
    let cards: Vec<_> = (0..12)
        .filter_map(|i| cx.debug_bounds(format!("pb-item-{i}").leak()))
        .collect();
    assert!(cards.len() >= 2, "{label}: cards drawn");
    assert!(
        cards.iter().all(|b| b.size == cards[0].size),
        "{label}: equal"
    );
    let area = cx.debug_bounds("pb-items-scroll").expect("grid drawn");
    let row: Vec<_> = cards
        .iter()
        .filter(|b| b.origin.y == cards[0].origin.y)
        .collect();
    let left = row[0].origin.x - area.origin.x;
    let right = area.right() - row[row.len() - 1].right();
    assert!(
        (left - right).abs() <= px(8.),
        "{label}: even margins {left:?} {right:?}"
    );
    assert!(area.right() <= cx.debug_bounds("pb-view").expect("view").right());
}

/// The wheel over the grid scrolls the grid only: the page title and
/// the toolbar stay where they are.
#[gpui::test]
fn only_the_grid_scrolls(cx: &mut TestAppContext) {
    let (state, mut cx) = window(cx, false);
    page(&state, 200, (900., 600.), &mut cx);
    let new = cx.debug_bounds("pb-items-new").expect("new drawn");
    let first = cx.debug_bounds("pb-item-0").expect("card drawn");
    let area = cx.debug_bounds("pb-items-scroll").expect("grid drawn");
    let view = cx.debug_bounds("pb-view").expect("view");
    assert!(
        area.bottom() <= view.bottom() + px(1.),
        "grid ends inside the window"
    );
    cx.simulate_event(ScrollEvent {
        position: area.center(),
        delta: ScrollDelta::Pixels(point(px(0.), px(-600.))),
        modifiers: Modifiers::none(),
        ..Default::default()
    });
    cx.run_until_parked();
    assert_eq!(
        cx.debug_bounds("pb-items-new"),
        Some(new),
        "page did not move"
    );
    let moved = cx.debug_bounds("pb-item-0").map(|b| b.origin.y);
    assert_ne!(moved, Some(first.origin.y), "grid scrolled");
}
