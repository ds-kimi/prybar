//! What blocks Steam, if anything, with what to do about it.

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::super::widgets::Tipped as _;
use super::super::super::widgets::{self, Kind};
use crate::app::Root;
use crate::state::AppState;
use crate::theme::prybar::Tokens;
use prybar_workshop::SteamStatus;

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
    // Asking again cannot add Workshop support to a build without it.
    let fixed_build = matches!(status, Some(SteamStatus::Disabled));
    // A build without Workshop is a fact, not an error to fix.
    let color = if ok == Some(false) && !fixed_build {
        t.danger
    } else {
        t.muted
    };
    div()
        .debug_selector(|| "pb-pub-steam".into())
        .flex()
        .items_center()
        .gap_2()
        .child(div().flex_1().text_color(color).child(text))
        .when(!fixed_build, |el| el.child(retry(status.is_some(), t, cx)))
}

fn retry(asked: bool, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    widgets::button("pb-pub-retry", "Retry", Kind::Secondary, asked, t)
        .debug_selector(|| "pb-pub-retry".into())
        .tip("Ask Steam again, after starting it or logging in", t)
        .on_click(cx.listener(|this, _, _, cx| this.state.update(cx, |s, cx| s.refresh_steam(cx))))
}
