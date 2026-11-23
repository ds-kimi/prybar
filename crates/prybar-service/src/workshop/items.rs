//! The user's items searched, filtered and sorted exactly like the
//! publish screen's list, by the same catalog code.

use prybar_workshop::catalog::{self, Query, Sort};
use prybar_workshop::spec::Visibility;
use prybar_workshop::ListedItem;
use serde::Serialize;

use crate::{Result, ServiceError};

#[derive(Debug, Clone, Default)]
pub struct ItemQuery {
    /// Case-insensitive match on title, tags and id.
    pub search: String,
    /// `subscribed`, `rated`, `favorited`, `updated` (default), `newest`, `title`.
    pub sort: Option<String>,
    /// Reverse the order: smallest first, or Z to A for `title`.
    pub reverse: bool,
    /// `public`, `friendsonly`, `private` or `unlisted`.
    pub visibility: Option<String>,
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ItemsReport {
    /// Items before filtering.
    pub total: usize,
    /// Every tag in use, for `tag`.
    pub tags: Vec<String>,
    pub items: Vec<ListedItem>,
}

/// Lists, records today's stats point (so the history chart fills the
/// same way from the CLI as from the GUI), then applies `q`.
pub fn workshop_items(app_id: u32, q: &ItemQuery) -> Result<ItemsReport> {
    let query = query(q)?;
    let all = crate::tools::workshop_list(app_id)?;
    // A lost point only costs one dot on the chart.
    let _ =
        prybar_workshop::item::record(&super::history::stats_file(), &all, super::history::today());
    let items = catalog::apply(&all, &query)
        .into_iter()
        .filter_map(|i| all.get(i).cloned())
        .collect();
    Ok(ItemsReport {
        total: all.len(),
        tags: catalog::tags(&all),
        items,
    })
}

fn query(q: &ItemQuery) -> Result<Query> {
    let sort = match q.sort.as_deref() {
        None => Sort::default(),
        Some(s) => *Sort::ALL
            .iter()
            .find(|x| x.key().eq_ignore_ascii_case(s))
            .ok_or_else(|| ServiceError::Usage(format!("unknown sort `{s}`: use subscribed, rated, favorited, updated, newest or title")))?,
    };
    let visibility = q
        .visibility
        .as_deref()
        .map(|v| {
            serde_json::from_value::<Visibility>(serde_json::Value::String(v.to_ascii_lowercase()))
                .map_err(|_| {
                    ServiceError::Usage(format!(
                        "unknown visibility `{v}`: use public, friendsonly, private or unlisted"
                    ))
                })
        })
        .transpose()?;
    // Title reads A to Z by default, numbers largest first.
    let descending = (sort != Sort::Title) != q.reverse;
    Ok(Query {
        text: q.search.clone(),
        sort,
        descending,
        visibility,
        tag: q.tag.clone(),
    })
}
