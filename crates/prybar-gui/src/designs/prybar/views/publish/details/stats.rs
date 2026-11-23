use std::time::SystemTime;

use gpui::{div, prelude::*};
use prybar_workshop::ListedItem;

use super::super::super::super::widgets::Tipped as _;
use crate::theme::prybar::Tokens;

/// The totals Steam gives, each with what it counts on hover.
pub fn view(it: &ListedItem, read: Option<SystemTime>, t: Tokens) -> impl IntoElement {
    let from = from(read);
    div()
        .flex()
        .flex_wrap()
        .children(rows(it).enumerate().map(|(i, r)| tile(i, r, &from, t)))
}

/// Name and what it counts, in the order [`rows`] gives the values.
const ABOUT: [(&str, &str); 7] = [
    (
        "Subscribers",
        "Current subscriptions, people who unsubscribed not counted",
    ),
    ("Unique subs", "Every account that ever subscribed"),
    ("Favorites", "Accounts that added it to their favorites"),
    ("Votes up", "Thumbs up on the Workshop page"),
    ("Votes down", "Thumbs down on the Workshop page"),
    ("Views", "Unique visitors of the Workshop page"),
    ("Comments", "Comments on the Workshop page"),
];

fn rows(it: &ListedItem) -> impl Iterator<Item = (&'static str, u64, &'static str)> {
    let values = [
        it.subscriptions,
        it.unique_subscriptions,
        it.favorites,
        u64::from(it.votes_up),
        u64::from(it.votes_down),
        it.views,
        it.comments,
    ];
    ABOUT
        .into_iter()
        .zip(values)
        .map(|((l, tip), v)| (l, v, tip))
}

fn tile(
    i: usize,
    (label, value, tip): (&'static str, u64, &'static str),
    from: &str,
    t: Tokens,
) -> impl IntoElement {
    div()
        .id(("pb-stat", i))
        .w_1_3()
        .p_1()
        .flex()
        .flex_col()
        .child(div().text_xs().text_color(t.muted).child(label))
        .child(div().child(value.to_string()))
        .tip(format!("{tip}.\n{from}"), t)
}

/// Where the numbers come from and how fresh they are, since Steam is
/// the only source and they only change when the list is read again.
fn from(read: Option<SystemTime>) -> String {
    let secs = read.and_then(|r| SystemTime::now().duration_since(r).ok());
    match secs.map(|d| d.as_secs()) {
        None => "From Steam.".to_owned(),
        Some(0..=59) => "From Steam, read just now.".to_owned(),
        Some(s @ 60..=3599) => format!("From Steam, read {} min ago.", s / 60),
        Some(s) => format!("From Steam, read {} h ago.", s / 3600),
    }
}
