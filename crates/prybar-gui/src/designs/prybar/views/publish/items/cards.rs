//! The items themselves, in a virtual list so only the rows on screen
//! are built and only their previews are fetched.

use std::collections::HashMap;
use std::ops::Range;
use std::rc::Rc;

use gpui::{div, prelude::*, px, uniform_list, AnyElement, App, Context, Entity};
use prybar_workshop::ListedItem;

use super::card::card;
use super::layout::{self, Layout, GAP};
use crate::app::Root;
use crate::state::tools::Thumb;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

/// What every row needs, shared by the list's draw callback.
struct Rows {
    shown: Vec<ListedItem>,
    thumbs: HashMap<u64, Thumb>,
    selected: Option<u64>,
    at: Layout,
    root: Entity<Root>,
    t: Tokens,
}

/// `width` is what the grid measured last frame (0 before the first).
pub fn view(
    s: &AppState,
    order: Vec<usize>,
    width: f32,
    t: Tokens,
    cx: &Context<Root>,
) -> AnyElement {
    let c = &s.tools.steam.catalog;
    let at = [layout::list, layout::grid][usize::from(c.grid)](width);
    let shown = order.iter().map(|&i| s.tools.steam.items[i].clone());
    let rows = Rc::new(Rows {
        shown: shown.collect(),
        thumbs: c.thumbs.clone(),
        selected: s.tools.item.selected,
        at,
        root: cx.entity(),
        t,
    });
    let count = rows.shown.len().div_ceil(at.cols);
    // Only this list scrolls: it fills the room left under the toolbar.
    uniform_list("pb-items", count, move |range, _, cx| {
        draw(&rows, range, cx)
    })
    .debug_selector(|| "pb-items-scroll".into())
    .size_full()
    .into_any_element()
}

fn draw(rows: &Rows, range: Range<usize>, cx: &mut App) -> Vec<AnyElement> {
    let mut want = Vec::new();
    let out = range.map(|r| {
        let at = rows.at;
        let end = ((r + 1) * at.cols).min(rows.shown.len());
        let cells = (r * at.cols..end).map(|i| {
            let item = &rows.shown[i];
            let thumb = rows.thumbs.get(&item.item_id);
            if thumb.is_none() {
                want.push((item.item_id, item.preview_url.clone()));
            }
            let on = rows.selected == Some(item.item_id);
            card(i, item, thumb, (at, on), &rows.root, rows.t)
        });
        let row = div()
            .h(px(at.row))
            .flex()
            .justify_center()
            .gap(px(GAP))
            .children(cells)
            // Empty slots keep a short last row on the same columns.
            .children((end..(r + 1) * at.cols).map(|_| div().flex_none().w(px(at.card.0))));
        row.into_any_element()
    });
    let out = out.collect();
    ask(&rows.root, want, cx);
    out
}

/// Deferred: the list draws while the state may be borrowed.
fn ask(root: &Entity<Root>, want: Vec<(u64, Option<String>)>, cx: &mut App) {
    if want.is_empty() {
        return;
    }
    let root = root.clone();
    cx.defer(move |cx| {
        let state = root.read(cx).state.clone();
        state.update(cx, |s, cx| s.want_thumbs(want, cx));
    });
}
