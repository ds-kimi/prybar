//! "My items": the user's Garry's Mod items as a searchable grid or
//! list, with a state for every reason it can be empty.

mod card;
mod cards;
mod chips;
mod text;
mod toolbar;

use gpui::{div, prelude::*, AnyElement, Context};
use prybar_workshop::catalog::apply;
use prybar_workshop::SteamStatus;

use super::super::super::widgets::Tipped as _;
use super::super::super::widgets::{self, Kind};
use crate::app::{Forms, Root};
use crate::state::tools::SteamView;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let steam = &s.tools.steam;
    let order = apply(&steam.items, &steam.catalog.query);
    let count = match (order.len(), steam.items.len()) {
        (0, 0) => String::new(),
        (a, b) if a == b => format!("{b} items"),
        (a, b) => format!("{a} of {b} items"),
    };
    let new = widgets::button("pb-items-new", "New item", Kind::Primary, true, t)
        .debug_selector(|| "pb-items-new".into())
        .tip("Start a new Workshop item from an empty form", t)
        .on_click(cx.listener(|this, _, _, cx| this.new_item(cx)));
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(widgets::section("Your items", t).pt_0())
                .child(div().flex_1().text_xs().text_color(t.muted).child(count))
                .child(new),
        )
        .child(toolbar::view(s, f, t, cx))
        .child(body(s, order, t, cx))
        .into_any_element()
}

/// The list, or the one sentence that says why there is none and what
/// to do about it.
fn body(s: &AppState, order: Vec<usize>, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let steam = &s.tools.steam;
    let say = |text: String| {
        div()
            .debug_selector(|| "pb-items-state".into())
            .py_6()
            .text_color(t.muted)
            .child(text)
            .into_any_element()
    };
    let why = empty_reason(steam, order.is_empty());
    if let Some(text) = why {
        return say(text);
    }
    cards::view(s, order, t, cx)
}

/// Why nothing is listed, or `None` when there is something to list.
fn empty_reason(steam: &SteamView, filtered_out: bool) -> Option<String> {
    let why = match &steam.status {
        None => Some("Loading your items from Steam…".to_owned()),
        Some(SteamStatus::Ready { .. }) => None,
        Some(_) => Some("Your items show here once Steam is ready, see above.".to_owned()),
    };
    let why = why.or_else(|| {
        let err = steam.items_error.as_ref();
        err.map(|e| {
            format!("Steam did not return your items: {e} Click the Steam badge to try again.")
        })
    });
    let why = why.or_else(|| {
        steam.items.is_empty().then(|| {
            "You have no Garry's Mod items yet. Click New item to publish your first one."
                .to_owned()
        })
    });
    why.or_else(|| {
        filtered_out.then(|| {
            "No item matches. Clear the search, pick Any visibility or click the lit tag again."
                .to_owned()
        })
    })
}
