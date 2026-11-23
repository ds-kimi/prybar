//! The Workshop page under load: many items, small and large windows,
//! a busy Steam.

mod busy;
mod games;
mod grid;

use gpui::{Entity, VisualTestContext};
use prybar_workshop::ListedItem;

use crate::state::{AppState, Screen};

/// `n` items in the list (ids 7, 8, 9, …), the page open at `w`×`h`.
pub(super) fn page(
    state: &Entity<AppState>,
    n: u64,
    (w, h): (f32, f32),
    cx: &mut VisualTestContext,
) {
    cx.simulate_resize(gpui::size(gpui::px(w), gpui::px(h)));
    state.update(cx, |s, cx| {
        let base = crate::state::tests::fake::item();
        s.tools.steam.items = (0..n)
            .map(|i| ListedItem {
                item_id: 7 + i,
                title: format!(
                    "{} {i} with a title far too long to fit on one card",
                    base.title
                ),
                ..base.clone()
            })
            .collect();
        s.navigate(Screen::Publish, cx);
    });
    // Twice: the first frame measures the grid, the second uses it.
    cx.run_until_parked();
    cx.update(|w, _| w.refresh());
    cx.run_until_parked();
}
