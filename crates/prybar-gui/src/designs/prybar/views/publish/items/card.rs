//! One item: preview, title and numbers; a grid card or a list row.

use gpui::{div, img, prelude::*, px, AnyElement, Div, Entity, ObjectFit, Stateful};
use prybar_workshop::ListedItem;

use super::super::super::super::widgets::Tipped as _;
use super::text::{details, words};
use crate::app::Root;
use crate::state::tools::Thumb;
use crate::theme::prybar::Tokens;

pub fn card(
    i: usize,
    it: &ListedItem,
    thumb: Option<&Thumb>,
    grid: bool,
    root: &Entity<Root>,
    t: Tokens,
) -> AnyElement {
    let (root, picked) = (root.clone(), it.clone());
    frame(i, grid, t)
        .tip(details(it), t)
        .on_click(move |_, _, cx| root.update(cx, |this, cx| this.edit_item(&picked, cx)))
        .child(picture(thumb, grid, t))
        .child(words(it, t))
        .into_any_element()
}

/// A lighter fill and a soft shadow lift the card on hover without
/// moving it, so the grid never jitters under the pointer.
fn frame(i: usize, grid: bool, t: Tokens) -> Stateful<Div> {
    div()
        .id(("pb-item", i))
        .debug_selector(move || format!("pb-item-{i}"))
        .flex_1()
        .w_0()
        .min_w_0()
        .p_1p5()
        .gap_1p5()
        .flex()
        .when(grid, |el| el.flex_col())
        .when(!grid, |el| el.items_center())
        .rounded_lg()
        .border_1()
        .border_color(t.border)
        .bg(t.surface)
        .cursor_pointer()
        .hover(move |s| s.bg(t.raised).border_color(t.muted).shadow_md())
        .active(move |s| s.bg(t.active))
}

/// The cached preview, or a plain tile while it loads or when there is none.
fn picture(thumb: Option<&Thumb>, grid: bool, t: Tokens) -> impl IntoElement {
    let pic = match thumb {
        Some(Thumb::Ready(path)) => img(path.clone())
            .size_full()
            .object_fit(ObjectFit::Cover)
            .into_any_element(),
        _ => div().size_full().into_any_element(),
    };
    let frame = div()
        .rounded_md()
        .overflow_hidden()
        .flex_none()
        .bg(t.active)
        .child(pic);
    if grid {
        frame.w_full().h(px(120.))
    } else {
        frame.size(px(36.))
    }
}
