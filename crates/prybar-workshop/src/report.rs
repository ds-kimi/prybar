use serde::Serialize;

use crate::spec::Visibility;

/// Upload progress as Steam reports it; `total` is 0 while unknown.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Progress {
    pub stage: String,
    pub done: u64,
    pub total: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PublishReport {
    pub dry_run: bool,
    /// `None` only for a dry-run create: no id exists yet.
    pub item_id: Option<u64>,
    pub created: bool,
    /// The item stays hidden until the owner accepts the agreement once.
    pub needs_legal_agreement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ListedItem {
    pub item_id: u64,
    pub title: String,
    pub tags: Vec<String>,
    pub time_updated: u32,
    pub time_created: u32,
    pub visibility: Visibility,
    pub subscriptions: u64,
    pub favorites: u64,
    pub votes_up: u32,
    pub votes_down: u32,
    pub preview_url: Option<String>,
}

/// Each state names the one thing the user must do next, so the UI can
/// say it instead of a generic "Steam error".
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum SteamStatus {
    /// Built without the `workshop` feature.
    Disabled,
    NotRunning,
    NotLoggedIn,
    NotOwned,
    Ready {
        user: String,
    },
}
