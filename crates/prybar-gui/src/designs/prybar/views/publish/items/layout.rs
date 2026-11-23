//! Card sizes from the width the grid really has, so every card is the
//! same size, gutters are equal and the row fills the page.

use std::cell::Cell;
use std::rc::Rc;

use gpui::{canvas, prelude::*};

/// Gutter between cards and rows, on the 8 px grid.
pub const GAP: f32 = 16.;
/// Inner padding of a card.
pub const PAD: f32 = 8.;
/// Narrowest card that still reads: a title and two lines of numbers.
const MIN_CARD: f32 = 176.;
/// Title line plus two small lines under the picture.
const WORDS: f32 = 56.;
/// Used until the grid has been measured once.
const GUESS: f32 = 640.;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Layout {
    pub cols: usize,
    /// Width and height of one card or row.
    pub card: (f32, f32),
    /// Height of the picture box: fixed, so a loading preview never
    /// moves the words under it.
    pub pic: f32,
    /// Card height plus the gutter under it.
    pub row: f32,
}

/// As many columns of at least [`MIN_CARD`] as fit `width`; the cards
/// share what is left equally, rounded down to whole pixels.
pub fn grid(width: f32) -> Layout {
    let w = if width > 0. { width } else { GUESS }.max(MIN_CARD);
    let cols = ((w + GAP) / (MIN_CARD + GAP)).floor().max(1.);
    let card = ((w - GAP * (cols - 1.)) / cols).floor();
    // 16:9, the shape most Workshop previews are cropped to on Steam.
    let pic = ((card - 2. * PAD) * 9. / 16.).round();
    let high = pic + WORDS + 3. * PAD;
    Layout {
        cols: cols as usize,
        card: (card, high),
        pic,
        row: high + GAP,
    }
}

/// One full-width row per item, every row the same height.
pub fn list(width: f32) -> Layout {
    let w = if width > 0. { width } else { GUESS }.floor();
    let high = 56.;
    Layout {
        cols: 1,
        card: (w, high),
        pic: 40.,
        row: high + PAD,
    }
}

/// Records the width the grid gets, laid out under it, and redraws when
/// it changed so the columns follow the window and the details panel.
pub fn measure(width: Rc<Cell<f32>>) -> impl IntoElement {
    let seen = move |b: gpui::Bounds<gpui::Pixels>, w: &mut gpui::Window, _: &mut gpui::App| {
        let now = f32::from(b.size.width);
        if (width.get() - now).abs() > 0.5 {
            width.set(now);
            w.refresh();
        }
    };
    canvas(seen, |_, (), _, _| {}).absolute().size_full()
}
