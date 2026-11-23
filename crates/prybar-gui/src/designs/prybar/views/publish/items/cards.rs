//! The items themselves, in a virtual list so only the rows on screen
//! are built and only their previews are fetched.

use std::collections::HashMap;
use std::ops::Range;
use std::rc::Rc;

use gpui::{div, prelude::*, px, uniform_list, AnyElement, App, Context, Entity};
use prybar_workshop::ListedItem;

use super::card::card;
use crate::app::Root;
use crate::state::tools::Thumb;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

const COLUMNS: usize = 4;

/// What every row needs, shared by the list's draw callback.
struct Rows {
    shown: Vec<ListedItem>,
    thumbs: HashMap<u64, Thumb>,
    selected: Option<u64>,
    cols: usize,
    height: f32,
    root: Entity<Root>,
    t: Tokens,
}

pub fn view(s: &AppState, order: Vec<usize>, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let c = &s.tools.steam.catalog;
    // The details panel takes a column's room when an item is picked.
    let wide = COLUMNS - usize::from(s.tools.item.selected.is_some());
    let (cols, height) = if c.grid { (wide, 212.) } else { (1, 52.) };
    let rows = Rc::new(Rows {
        shown: order
            .iter()
            .map(|&i| s.tools.steam.items[i].clone())
            .collect(),
        thumbs: c.thumbs.clone(),
        selected: s.tools.item.selected,
        cols,
        height,
        root: cx.entity(),
        t,
    });
    let count = rows.shown.len().div_ceil(cols);
    uniform_list("pb-items", count, move |range, _, cx| {
        draw(&rows, range, cx)
    })
    // As tall as its rows, up to about two grid rows or eight list rows.
    .h(px((count as f32 * height).min(if c.grid {
        440.
    } else {
        416.
    })))
    .into_any_element()
}

fn draw(rows: &Rows, range: Range<usize>, cx: &mut App) -> Vec<AnyElement> {
    let mut want = Vec::new();
    let out = range.map(|r| {
        let end = ((r + 1) * rows.cols).min(rows.shown.len());
        let cells = (r * rows.cols..end).map(|i| {
            let item = &rows.shown[i];
            let thumb = rows.thumbs.get(&item.item_id);
            if thumb.is_none() {
                want.push((item.item_id, item.preview_url.clone()));
            }
            let on = rows.selected == Some(item.item_id);
            card(i, item, thumb, (rows.cols > 1, on), &rows.root, rows.t)
        });
        let row = div()
            .h(px(rows.height))
            .flex()
            .gap_2()
            .pb_2()
            .children(cells)
            // Empty slots keep a short last row on the same columns.
            .children((end..(r + 1) * rows.cols).map(|_| div().flex_1().w_0()));
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
