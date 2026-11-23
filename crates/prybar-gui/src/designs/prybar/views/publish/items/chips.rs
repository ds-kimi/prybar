//! The tag filter: one chip per tag the user's items carry.

use gpui::{div, prelude::*, Context, Div, Stateful};
use prybar_workshop::catalog::tags;

use super::super::super::super::widgets::{self, Tipped as _};
use crate::app::Root;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let picked = s.tools.steam.catalog.query.tag.clone();
    let all = tags(&s.tools.steam.items).into_iter().enumerate();
    div()
        .flex()
        .flex_wrap()
        .gap_1()
        .children(all.map(|(i, tag)| {
            let on = picked
                .as_ref()
                .is_some_and(|p| p.eq_ignore_ascii_case(&tag));
            chip(i, tag, on, t, cx)
        }))
}

/// Lit chips are rust; clicking a lit one clears the filter.
fn chip(i: usize, tag: String, on: bool, t: Tokens, cx: &Context<Root>) -> Stateful<Div> {
    let next = (!on).then(|| tag.clone());
    let tip = if on {
        "Show every tag again"
    } else {
        "Only items with this tag"
    };
    widgets::ring(div().id(("pb-items-tag", i)), t)
        .debug_selector(move || format!("pb-items-tag-{i}"))
        .px_2()
        .rounded_md()
        .text_xs()
        .cursor_pointer()
        .bg(if on { t.active } else { t.surface })
        .text_color(if on { t.rust } else { t.muted })
        .hover(move |st| st.bg(t.hover))
        .tip(tip, t)
        .on_click(cx.listener(move |this, _, _, cx| {
            let next = next.clone();
            this.state
                .update(cx, |s, cx| s.set_items(|c| c.query.tag = next, cx))
        }))
        .child(tag)
}
