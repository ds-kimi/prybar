use crate::{Result, ServiceError};
use prybar_workshop::spec::ItemSpec;
use prybar_workshop::{ListedItem, Progress, PublishReport, WorkshopError};

pub fn publish(
    spec: &ItemSpec,
    dry_run: bool,
    progress: &mut dyn FnMut(&Progress),
) -> Result<PublishReport> {
    prybar_workshop::publish(spec, dry_run, progress).map_err(workshop_error)
}

pub fn workshop_list(app_id: u32) -> Result<Vec<ListedItem>> {
    prybar_workshop::list_items(app_id).map_err(workshop_error)
}

fn workshop_error(e: WorkshopError) -> ServiceError {
    match e {
        WorkshopError::Invalid { .. } => ServiceError::invalid("workshop item", e),
        WorkshopError::Io { path, source } => ServiceError::Io { path, source },
        _ => ServiceError::Tool(e.to_string()),
    }
}
