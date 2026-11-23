//! The edit popup fits every window, and toasts move as designed.

mod toasts;

use gpui::{px, size, Bounds, Pixels, TestAppContext};

use super::window;

/// The small, default and large windows the popup is checked at.
const SIZES: [(f32, f32); 3] = [(900., 600.), (1280., 800.), (1800., 1100.)];

fn inside(b: Bounds<Pixels>, w: f32, h: f32) -> bool {
    b.origin.x >= px(0.) && b.origin.y >= px(0.) && b.right() <= px(w) && b.bottom() <= px(h)
}

/// Card, close button and the pinned actions stay inside the window,
/// the form splits into columns only when there is room.
#[gpui::test]
fn edit_popup_fits_every_window(cx: &mut TestAppContext) {
    for (w, h) in SIZES {
        let (_state, mut cx) = window(cx, false);
        cx.simulate_resize(size(px(w), px(h)));
        let item = Some(crate::state::tests::fake::item());
        cx.update(|win, cx| {
            let root = win.root::<crate::app::Root>().flatten().expect("root");
            root.update(cx, |r, cx| r.open_publish(item.clone(), win, cx));
        });
        cx.run_until_parked();
        for id in ["pb-modal", "pb-modal-close", "pb-modal-footer", "pb-pub-go"] {
            let b = cx.debug_bounds(id).unwrap_or_else(|| panic!("{id} drawn"));
            assert!(inside(b, w, h), "{id} inside {w}x{h}: {b:?}");
        }
        let card = cx.debug_bounds("pb-modal").expect("card");
        let pic = cx.debug_bounds("pb-pub-picture").expect("picture");
        let wide = crate::designs::modal::wide(size(px(w), px(h)));
        assert_eq!(pic.origin.x > card.center().x, wide, "columns at {w}x{h}");
        assert!(pic.right() <= card.right(), "picture inside the card");
    }
}
