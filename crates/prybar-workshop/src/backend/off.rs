use crate::spec::ItemSpec;
use crate::{ListedItem, Progress, PublishReport, WorkshopError};

pub fn publish(
    _spec: &ItemSpec,
    _progress: &mut dyn FnMut(&Progress),
) -> Result<PublishReport, WorkshopError> {
    Err(WorkshopError::Disabled)
}

pub fn list(_app_id: u32) -> Result<Vec<ListedItem>, WorkshopError> {
    Err(WorkshopError::Disabled)
}
