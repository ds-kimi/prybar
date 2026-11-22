//! The orders the item list offers, named the way Steam names them.

use std::cmp::Ordering;

use crate::ListedItem;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Sort {
    Subscribed,
    Rated,
    Favorited,
    #[default]
    Updated,
    Newest,
    Title,
}

impl Sort {
    pub const ALL: [Sort; 6] = [
        Sort::Subscribed,
        Sort::Rated,
        Sort::Favorited,
        Sort::Updated,
        Sort::Newest,
        Sort::Title,
    ];

    /// The word saved in the prefs file; [`Sort::ALL`] order.
    pub fn key(self) -> &'static str {
        [
            "subscribed",
            "rated",
            "favorited",
            "updated",
            "newest",
            "title",
        ][Sort::ALL.iter().position(|s| *s == self).unwrap_or(3)]
    }
}

/// Share of up votes, 0 to 1000; no votes counts as the middle so a new
/// item sits between liked and disliked ones.
pub fn rating(item: &ListedItem) -> u32 {
    let all = u64::from(item.votes_up) + u64::from(item.votes_down);
    if all == 0 {
        return 500;
    }
    (u64::from(item.votes_up) * 1000 / all) as u32
}

/// Ascending; the caller reverses for descending.
pub(super) fn order(a: &ListedItem, b: &ListedItem, sort: Sort) -> Ordering {
    match sort {
        Sort::Subscribed => a.subscriptions.cmp(&b.subscriptions),
        Sort::Rated => (rating(a), a.votes_up).cmp(&(rating(b), b.votes_up)),
        Sort::Favorited => a.favorites.cmp(&b.favorites),
        Sort::Updated => a.time_updated.cmp(&b.time_updated),
        Sort::Newest => a.time_created.cmp(&b.time_created),
        Sort::Title => a.title.to_lowercase().cmp(&b.title.to_lowercase()),
    }
}
