//! One item: preview, title and numbers; a grid card or a list row,
//! always exactly the size the layout gives.

use std::time::Duration;

use gpui::{
    div, img, prelude::*, px, Animation, AnimationExt as _, AnyElement, Div, Entity, ObjectFit,
    Stateful,
};
use prybar_workshop::ListedItem;

use super::super::super::super::widgets::Tipped as _;
use super::layout::{Layout, PAD};
use super::text::{details, words};
use crate::app::Root;
use crate::state::tools::Thumb;
use crate::theme::prybar::Tokens;

pub fn card(
    i: usize,
    it: &ListedItem,
    thumb: Option<&Thumb>,
    (at, on): (Layout, bool),
    root: &Entity<Root>,
    t: Tokens,
) -> AnyElement {
    let (root, id) = (root.clone(), it.item_id);
    frame(i, at, t)
        .when(on, |el| el.border_color(t.rust).bg(t.raised))
        .tip(details(it), t)
        .on_click(move |_, _, cx| {
            let state = root.read(cx).state.clone();
            state.update(cx, |s, cx| s.select_item(id, cx))
        })
        .child(picture(i, thumb, at, t))
        .child(words(it, t))
        .into_any_element()
}

/// A lighter fill and a soft shadow lift the card on hover without
/// moving it, so the grid never jitters under the pointer.
fn frame(i: usize, at: Layout, t: Tokens) -> Stateful<Div> {
    let grid = at.cols > 1 || at.pic > 40.;
    div()
        .id(("pb-item", i))
        .debug_selector(move || format!("pb-item-{i}"))
        .flex_none()
        .w(px(at.card.0))
        .h(px(at.card.1))
        .overflow_hidden()
        .p(px(PAD))
        .gap(px(PAD))
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

/// The cached preview in a box of fixed size; a soft pulse while it
/// loads, a plain tile when there is none.
fn picture(i: usize, thumb: Option<&Thumb>, at: Layout, t: Tokens) -> AnyElement {
    let grid = at.pic > 40.;
    let boxed = div()
        .rounded_md()
        .overflow_hidden()
        .flex_none()
        .bg(t.active)
        .when(grid, |el| el.w_full().h(px(at.pic)))
        .when(!grid, |el| el.size(px(at.pic)));
    match thumb {
        Some(Thumb::Ready(path)) => boxed
            .child(img(path.clone()).size_full().object_fit(ObjectFit::Cover))
            .into_any_element(),
        Some(Thumb::Missing) => boxed.into_any_element(),
        _ => boxed
            .with_animation(
                ("pb-item-skel", i),
                Animation::new(Duration::from_millis(1200)).repeat(),
                |el, d| el.opacity(0.55 + 0.45 * (d * std::f32::consts::TAU).cos().abs()),
            )
            .into_any_element(),
    }
}
