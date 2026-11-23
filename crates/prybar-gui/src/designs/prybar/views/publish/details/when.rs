//! Small lookups the panel needs: which item, and dates as text.

use gpui::{div, prelude::*};
use prybar_workshop::ListedItem;

use crate::state::AppState;
use crate::theme::prybar::Tokens;

/// The item whose details show, while it is still listed.
pub fn selected(s: &AppState) -> Option<&ListedItem> {
    let id = s.tools.item.selected?;
    s.tools.steam.items.iter().find(|i| i.item_id == id)
}

/// Days since 1970 as an ISO date (Howard Hinnant's civil algorithm),
/// so no date crate is needed for one label.
pub fn date(day: u32) -> String {
    let z = i64::from(day) + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

/// A chart of fewer points, or of one day, draws a shape that means
/// nothing; it waits for at least three readings on two days or more.
pub fn chartable(series: &[prybar_workshop::item::Point]) -> bool {
    let days = |p: &prybar_workshop::item::Point| p.day;
    let spread = series.iter().map(days).max() > series.iter().map(days).min();
    series.len() >= 3 && spread
}

/// Muted small text, the panel's secondary voice.
pub fn small(s: String, t: Tokens) -> gpui::Div {
    div().text_xs().text_color(t.muted).child(s)
}
