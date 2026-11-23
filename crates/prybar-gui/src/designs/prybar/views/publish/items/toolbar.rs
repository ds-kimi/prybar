//! Search, sort and its direction, filters, and grid or list.

use gpui::{div, prelude::*, px, AnyElement, Context, Div, Stateful};

use super::super::super::super::widgets::Tipped as _;
use super::super::super::super::widgets::{self, Kind};
use crate::app::{Forms, Root};
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let items = &f.publish.items;
    let grid = s.tools.steam.catalog.grid;
    let row = div()
        .flex()
        .items_center()
        .gap_2()
        .child(div().flex_1().child(items.search.clone()))
        .child(div().w(px(170.)).child(items.sort.clone()))
        .child(order(s, t, cx))
        .child(div().w(px(150.)).child(items.filter.clone()))
        .child(layout("pb-items-grid", "Grid", true, grid, t, cx))
        .child(layout("pb-items-list", "List", false, grid, t, cx));
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(row)
        .child(super::chips::view(s, t, cx))
        .into_any_element()
}

fn order(s: &AppState, t: Tokens, cx: &Context<Root>) -> Stateful<Div> {
    let (arrow, tip) = if s.tools.steam.catalog.query.descending {
        ("↓", "Highest first; click for lowest first")
    } else {
        ("↑", "Lowest first; click for highest first")
    };
    widgets::button("pb-items-order", arrow, Kind::Secondary, true, t)
        .debug_selector(|| "pb-items-order".into())
        .tip(tip, t)
        .on_click(cx.listener(|this, _, _, cx| {
            this.state
                .update(cx, |s, cx| s.set_items(|c| c.query.descending ^= true, cx))
        }))
}

fn layout(
    id: &'static str,
    label: &'static str,
    grid: bool,
    now: bool,
    t: Tokens,
    cx: &Context<Root>,
) -> Stateful<Div> {
    let tip = if grid {
        "Show previews in a grid"
    } else {
        "Show a compact list"
    };
    widgets::button(id, label, Kind::Secondary, true, t)
        .debug_selector(move || id.into())
        .when(now == grid, |el| el.bg(t.active))
        .tip(tip, t)
        .on_click(cx.listener(move |this, _, _, cx| {
            this.state
                .update(cx, |s, cx| s.set_items(|c| c.grid = grid, cx))
        }))
}
