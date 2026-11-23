//! The Workshop page: the user's items with the selected one's details;
//! creating and editing happen in popups.

mod addon;
mod details;
mod fields;
mod game;
mod items;
mod popup;
mod result;
mod steam;

pub use popup::{footer, view as popup};

use gpui::{div, prelude::*, px, AnyElement, Context};

use super::super::widgets;
use crate::app::{Forms, Root};
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let none = s
        .tools
        .steam
        .games
        .as_ref()
        .is_some_and(|g| g.detected.is_empty());
    div()
        .max_w(px(1040.))
        .size_full()
        .min_h_0()
        .flex()
        .flex_col()
        .gap_3()
        .child(header(s, t, cx))
        .child(div().text_color(t.muted).child(intro(s)))
        .children(none.then(|| game::empty(t, cx)))
        .when(!none, |el| {
            el.child(steam::view(s, t, cx))
                .child(items::view(s, f, t, cx))
        })
        .into_any_element()
}

/// The game chips share the heading's line so the grid keeps its height.
fn header(s: &AppState, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .justify_between()
        .gap_2()
        .child(widgets::heading("Publish"))
        .children(game::chips(s, t, cx))
}

fn intro(s: &AppState) -> String {
    let target = s.tools.steam.target();
    let name = target.map_or("Workshop", |g| g.name);
    let rules = target.map_or("your content", |g| g.rules);
    format!("Your {name} items. Click one for its stats and history; New item publishes {rules}.")
}
