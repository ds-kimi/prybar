//! The words on a card and in its tooltip.

use gpui::{div, prelude::*};
use prybar_workshop::catalog::rating;
use prybar_workshop::spec::Visibility;
use prybar_workshop::ListedItem;

use crate::theme::prybar::Tokens;

/// Title, then the numbers a creator checks first.
pub fn words(it: &ListedItem, t: Tokens) -> impl IntoElement {
    let votes = u64::from(it.votes_up) + u64::from(it.votes_down);
    let score = match votes {
        0 => "no votes".to_owned(),
        _ => format!("{}% up", rating(it) / 10),
    };
    let vis = match it.visibility {
        Visibility::Public => "Public",
        Visibility::FriendsOnly => "Friends",
        Visibility::Private => "Hidden",
        Visibility::Unlisted => "Unlisted",
    };
    let small = |text: String| div().text_xs().text_color(t.muted).truncate().child(text);
    div()
        .min_w_0()
        .flex_1()
        .flex()
        .flex_col()
        .child(div().truncate().child(it.title.clone()))
        .child(small(format!("{} subs · {score}", it.subscriptions)))
        .child(small(format!("{vis} · updated {}", ago(it.time_updated))))
}

/// Coarse on purpose: a card has room for "3 d ago", not a date.
pub fn ago(when: u32) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let days = now.saturating_sub(u64::from(when)) / 86_400;
    match days {
        0 => "today".to_owned(),
        1..=59 => format!("{days} d ago"),
        60..=729 => format!("{} mo ago", days / 30),
        _ => format!("{} y ago", days / 365),
    }
}

/// Everything known about the item, for its tooltip.
pub fn details(it: &ListedItem) -> String {
    let tags = match it.tags.is_empty() {
        true => "none".to_owned(),
        false => it.tags.join(", "),
    };
    format!(
        "{}\nId {} · {} subscribers · {} favorites · {} up, {} down\nTags: {tags}\nClick for its stats and history.",
        it.title, it.item_id, it.subscriptions, it.favorites, it.votes_up, it.votes_down,
    )
}
