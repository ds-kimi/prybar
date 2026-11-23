//! "My items": the user's Garry's Mod items as a searchable grid or
//! list, with a state for every reason it can be empty.

mod card;
mod cards;
mod chips;
mod layout;
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
    let head = head(order.len(), steam.items.len(), t, cx);
    let width = f.items_width.clone();
    let main = div().relative().flex_1().min_w_0().h_full();
    let main = main.child(body(s, order, width.get(), t, cx));
    let main = main.child(layout::measure(width));
    div()
        .flex_1()
        .min_h_0()
        .flex()
        .flex_col()
        .gap_2()
        .child(head)
        .child(toolbar::view(s, f, t, cx))
        .child(
            // The grid and the details panel each scroll on their own;
            // this row only shares the height left on the page.
            div()
                .w_full()
                .flex_1()
                .min_h_0()
                .flex()
                .gap_4()
                .child(main)
                .children(super::details::selected(s).map(|it| super::details::view(s, it, t, cx))),
        )
        .into_any_element()
}

/// The title, how many items show, and New item.
fn head(shown: usize, all: usize, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let count = match (shown, all) {
        (0, 0) => String::new(),
        (a, b) if a == b => format!("{b} items"),
        (a, b) => format!("{a} of {b} items"),
    };
    let new = widgets::button("pb-items-new", "New item", Kind::Primary, true, t)
        .debug_selector(|| "pb-items-new".into())
        .tip("Publish a new addon to the Workshop", t)
        .on_click(cx.listener(|this, _, w, cx| this.open_publish(None, w, cx)));
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(widgets::section("Your items", t).pt_0())
        .child(div().flex_1().text_xs().text_color(t.muted).child(count))
        .child(new)
}

/// The list, or the one sentence that says why there is none and what
/// to do about it.
fn body(s: &AppState, order: Vec<usize>, width: f32, t: Tokens, cx: &Context<Root>) -> AnyElement {
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
    cards::view(s, order, width, t, cx)
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
            let name = steam.target().map_or("Workshop", |g| g.name);
            format!("You have no {name} items yet. Click New item to publish your first one.")
        })
    });
    why.or_else(|| {
        filtered_out.then(|| {
            "No item matches. Clear the search, pick Any visibility or click the lit tag again."
                .to_owned()
        })
    })
}
