//! One item in full: its current fields and stats, the daily stats points
//! recorded so far, and its version history from the Workshop page.

use std::path::PathBuf;

use prybar_workshop::ListedItem;
use serde::Serialize;

use crate::{Result, ServiceError};

#[derive(Debug, Clone, Serialize)]
pub struct StatPoint {
    /// Days since 1970-01-01 (UTC).
    pub day: u32,
    pub subscriptions: u64,
    pub favorites: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChangeEntry {
    pub time: Option<u64>,
    pub headline: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ItemReport {
    pub item: ListedItem,
    pub url: String,
    /// One point per day an item list was read (GUI or CLI).
    pub stats: Vec<StatPoint>,
    pub history: Vec<ChangeEntry>,
    /// Why the history is missing (Steam rate limit, offline), if it is.
    pub history_error: Option<String>,
    /// The history is an older copy because Steam did not answer.
    pub history_stale: bool,
}

pub fn workshop_item(app_id: u32, item_id: u64) -> Result<ItemReport> {
    let all = crate::tools::workshop_list(app_id)?;
    let _ = prybar_workshop::item::record(&stats_file(), &all, today());
    let item = all
        .into_iter()
        .find(|i| i.item_id == item_id)
        .ok_or_else(|| {
            ServiceError::NotFound(format!("Workshop item {item_id} among your items"))
        })?;
    let stats = prybar_workshop::item::series(&stats_file(), item_id)
        .into_iter()
        .map(|p| StatPoint {
            day: p.day,
            subscriptions: p.subscriptions,
            favorites: p.favorites,
        })
        .collect();
    let (history, history_error, history_stale) = history(item_id);
    Ok(ItemReport {
        item,
        url: format!("https://steamcommunity.com/sharedfiles/filedetails/?id={item_id}"),
        stats,
        history,
        history_error,
        history_stale,
    })
}

/// A missing history is reported, not an error: the item itself is
/// still worth returning when Steam rate-limits the changelog page.
fn history(item_id: u64) -> (Vec<ChangeEntry>, Option<String>, bool) {
    match prybar_workshop::changelog::cached(item_id, &super::data_dir().join("changelogs")) {
        Ok(l) => {
            let changes = l.changes.into_iter();
            let entries = changes.map(|c| ChangeEntry {
                time: c.time,
                headline: c.headline,
                note: c.note,
            });
            (entries.collect(), None, l.stale)
        }
        Err(f) => (Vec::new(), Some(f.message), false),
    }
}

/// The same file the GUI's chart reads.
pub(super) fn stats_file() -> PathBuf {
    super::data_dir().join("workshop-stats.tsv")
}

/// Days since 1970 in UTC, the unit snapshots are kept in.
pub(super) fn today() -> u32 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    u32::try_from(secs / 86_400).unwrap_or(u32::MAX)
}
