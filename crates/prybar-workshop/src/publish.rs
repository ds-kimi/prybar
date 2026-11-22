use crate::spec::{validate, ItemSpec};
use crate::stage::stage;
use crate::{steam, ListedItem, Progress, PublishReport, Steam, SteamStatus, WorkshopError};

/// [`publish_with`] on the real Steam backend.
pub fn publish(
    spec: &ItemSpec,
    dry_run: bool,
    progress: &mut dyn FnMut(&Progress),
) -> Result<PublishReport, WorkshopError> {
    publish_with(steam(), spec, dry_run, progress)
}

/// Validates, then creates or updates the item. `dry_run` stops after
/// validation and never touches Steam, so it works offline and without
/// the `workshop` feature.
pub fn publish_with(
    steam: &dyn Steam,
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
    // Say so before packing anything a build without Steam cannot send.
    if steam.status(spec.app_id) == SteamStatus::Disabled {
        return Err(WorkshopError::Disabled);
    }
    let (ready, _temp) = stage(spec)?;
    steam.upload(&ready, progress)
}

/// Items the logged-in Steam user published for `app_id` (first page).
pub fn list_items(app_id: u32) -> Result<Vec<ListedItem>, WorkshopError> {
    steam().list(app_id)
}

/// Is Steam running, logged in and owning `app_id`?
pub fn status(app_id: u32) -> SteamStatus {
    steam().status(app_id)
}
