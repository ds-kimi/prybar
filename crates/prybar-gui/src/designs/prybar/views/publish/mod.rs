//! The Workshop page: the user's items with the selected one's details;
//! creating and editing happen in popups.

mod addon;
mod details;
mod fields;
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
    div()
        .max_w(px(1040.))
        .size_full()
        .min_h_0()
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("Publish"))
        .child(
            div()
                .text_color(t.muted)
                .child("Your Garry's Mod Workshop items. Click one for its stats and history; New item publishes an addon folder or .gma."),
        )
        .child(steam::view(s, t, cx))
        .child(items::view(s, f, t, cx))
        .into_any_element()
}
