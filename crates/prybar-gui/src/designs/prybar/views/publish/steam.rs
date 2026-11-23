//! What blocks Steam, if anything, with what to do about it.

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::super::widgets::Tipped as _;
use super::super::super::widgets::{self, Kind};
use crate::app::Root;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        // Good news lives in the top bar badge; here only what blocks.
        .when(s.tools.steam.sentence().1 == Some(false), |el| {
            el.child(line(s, t, cx))
        })
        .into_any_element()
}

fn line(s: &AppState, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let status = s.tools.steam.status.as_ref();
    let (text, ok) = s.tools.steam.sentence();
    let retry = widgets::button(
        "pb-pub-retry",
        "Retry",
        Kind::Secondary,
        status.is_some(),
        t,
    )
    .debug_selector(|| "pb-pub-retry".into())
    .tip("Ask Steam again, after starting it or logging in", t)
    .on_click(cx.listener(|this, _, _, cx| this.state.update(cx, |s, cx| s.refresh_steam(cx))));
    div()
        .debug_selector(|| "pb-pub-steam".into())
        .flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .flex_1()
                .text_color(if ok == Some(false) { t.danger } else { t.muted })
                .child(text),
        )
        .child(retry)
}
