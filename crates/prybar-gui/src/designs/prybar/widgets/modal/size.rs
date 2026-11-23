//! How big the popup is for a given window: wide enough to read, never
//! wider or taller than the window minus a margin.

use gpui::{px, size, Pixels, Size};

/// Narrowest card that still fits a form comfortably.
const MIN_W: f32 = 480.;
/// Past this, lines get too long to scan.
const MAX_W: f32 = 880.;
/// Share of the window the card takes between the two bounds.
const SHARE: f32 = 0.68;
/// Kept clear around the card so the dimmed window still shows.
const MARGIN: f32 = 32.;
/// From this card width the form splits into two columns.
const TWO_COLUMNS: f32 = 720.;

/// The card's width and its maximum height in `window`.
pub fn card_size(window: Size<Pixels>) -> Size<Pixels> {
    let (w, h) = (f32::from(window.width), f32::from(window.height));
    let room = (w - 2. * MARGIN).max(0.);
    let width = (w * SHARE).clamp(MIN_W, MAX_W).min(room);
    size(px(width), px((h - 2. * MARGIN).max(0.)))
}

/// Whether the popup body has room for side-by-side columns.
pub fn wide(window: Size<Pixels>) -> bool {
    f32::from(card_size(window).width) >= TWO_COLUMNS
}
