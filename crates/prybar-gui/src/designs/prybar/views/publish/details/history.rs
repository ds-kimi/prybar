use gpui::{div, prelude::*, AnyElement, Context};

use super::super::super::super::widgets::Tipped as _;
use super::super::super::super::widgets::{self, Kind};
use crate::app::Root;
use crate::state::tools::History;
use crate::theme::prybar::Tokens;

pub fn view(h: Option<&History>, id: u64, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let small = |s: String| {
        div()
            .text_xs()
            .text_color(t.muted)
            .child(s)
            .into_any_element()
    };
    let retry = |why: String| retry(why, id, t, cx);
    let body = match h {
        None | Some(History::Waiting(_) | History::Loading(_)) => {
            vec![small("Reading the change history…".into())]
        }
        Some(History::Busy { wait, .. }) => {
            vec![retry(format!("Steam is busy, retrying in {wait} s."))]
        }
        Some(History::Failed(why)) => vec![retry(format!("No change history: {why}"))],
        Some(History::Ready(log)) => super::notes::view(log, t),
    };
    div()
        .debug_selector(|| "pb-history".into())
        .flex()
        .flex_col()
        .gap_2()
        .child(widgets::section("Version history", t).pt_0())
        .children(body)
}

/// A calm line saying what happened, with a way to ask again now.
fn retry(why: String, id: u64, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let again = widgets::button("pb-hist-retry", "Try again", Kind::Secondary, true, t)
        .debug_selector(|| "pb-hist-retry".into())
        .tip("Ask Steam for the change notes now", t)
        .on_click(cx.listener(move |this, _, _, cx| {
            this.state.update(cx, |s, cx| s.retry_history(id, cx))
        }));
    div()
        .debug_selector(|| "pb-hist-busy".into())
        .flex()
        .items_center()
        .gap_2()
        .child(div().flex_1().text_xs().text_color(t.muted).child(why))
        .child(again)
        .into_any_element()
}
