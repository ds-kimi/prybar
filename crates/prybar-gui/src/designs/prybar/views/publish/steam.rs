//! What blocks Steam, if anything, with what to do about it, then the items
//! the user already published, each one click away from an update.

use gpui::{div, prelude::*, AnyElement, Context};
use prybar_workshop::ListedItem;

use super::super::super::widgets::Tipped as _;
use super::super::super::widgets::{self, Kind};
use crate::app::Root;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let steam = &s.tools.steam;
    div()
        .flex()
        .flex_col()
        .gap_1()
        // Good news lives in the top bar badge; here only what blocks.
        .when(s.tools.steam.sentence().1 == Some(false), |el| {
            el.child(line(s, t, cx))
        })
        .when_some(steam.items_error.clone(), |el, why| {
            el.child(div().text_color(t.danger).child(why))
        })
        .when(!steam.items.is_empty(), |el| {
            el.child(items(&steam.items, t, cx))
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

/// Newest first, as Steam sorts them; a click fills id and title.
fn items(list: &[ListedItem], t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let rows = list.iter().take(50).enumerate().map(|(i, item)| {
        let (id, title) = (item.item_id, item.title.clone());
        let days = now.saturating_sub(u64::from(item.time_updated)) / 86_400;
        let meta = format!(
            "{:?} · {} subscribers · updated {days} d ago",
            item.visibility, item.subscriptions
        );
        widgets::row(("pb-pub-item", i), false, t)
            .debug_selector(move || format!("pb-pub-item-{i}"))
            .tip("Fill the form with this item, to update it", t)
            .child(div().flex_1().truncate().child(item.title.clone()))
            .child(div().text_xs().text_color(t.muted).child(meta))
            .on_click(cx.listener(move |this, _, _, cx| {
                let f = &this.forms.publish;
                f.item_id
                    .update(cx, |i, cx| i.set_text(&id.to_string(), cx));
                f.title.update(cx, |i, cx| i.set_text(&title, cx));
            }))
    });
    div()
        .flex()
        .flex_col()
        .child(widgets::section("Your items: pick one to update it", t))
        .children(rows)
}
