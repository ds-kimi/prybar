//! The user's item list as the publish screen shows it: searched,
//! filtered and sorted in memory, since a creator rarely has more than a
//! few hundred items and every keystroke must answer at once.

mod sort;

pub use sort::{rating, Sort};

use crate::spec::Visibility;
use crate::ListedItem;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Query {
    /// Matched, case-insensitive, against title, tags and the id.
    pub text: String,
    pub sort: Sort,
    pub descending: bool,
    pub visibility: Option<Visibility>,
    pub tag: Option<String>,
}

/// Indexes into `items` that pass the query, in display order. Ties keep
/// Steam's order so the list does not shuffle between redraws.
pub fn apply(items: &[ListedItem], q: &Query) -> Vec<usize> {
    let needle = q.text.trim().to_lowercase();
    let mut out: Vec<usize> = (0..items.len())
        .filter(|&i| keeps(&items[i], q, &needle))
        .collect();
    out.sort_by(|&a, &b| {
        let o = sort::order(&items[a], &items[b], q.sort);
        if q.descending {
            o.reverse()
        } else {
            o
        }
    });
    out
}

fn keeps(item: &ListedItem, q: &Query, needle: &str) -> bool {
    let vis = q.visibility.is_none_or(|v| v == item.visibility);
    let tag = q
        .tag
        .as_ref()
        .is_none_or(|t| item.tags.iter().any(|x| x.eq_ignore_ascii_case(t)));
    let text = needle.is_empty()
        || item.title.to_lowercase().contains(needle)
        || item.item_id.to_string().contains(needle)
        || item.tags.iter().any(|t| t.to_lowercase().contains(needle));
    vis && tag && text
}

/// Every tag in use, once, sorted, for the tag filter.
pub fn tags(items: &[ListedItem]) -> Vec<String> {
    let mut all: Vec<String> = items.iter().flat_map(|i| i.tags.clone()).collect();
    all.sort_by_key(|t| t.to_lowercase());
    all.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    all
}
