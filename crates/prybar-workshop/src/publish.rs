use serde::Serialize;

use crate::spec::{validate, ItemSpec};
use crate::{backend, WorkshopError};

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
}

/// Validates, then creates or updates the item. `dry_run` stops after
/// validation and never loads the Steam SDK, so it works offline and
/// without the `workshop` feature.
pub fn publish(
    spec: &ItemSpec,
    dry_run: bool,
    progress: &mut dyn FnMut(&Progress),
) -> Result<PublishReport, WorkshopError> {
    validate(spec)?;
    if dry_run {
        return Ok(PublishReport {
            dry_run,
            item_id: spec.item_id,
            created: spec.item_id.is_none(),
            needs_legal_agreement: false,
        });
    }
    backend::publish(spec, progress)
}

/// Items the logged-in Steam user published for `app_id` (first page).
pub fn list_items(app_id: u32) -> Result<Vec<ListedItem>, WorkshopError> {
    backend::list(app_id)
}
