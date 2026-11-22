use super::Steam;
use crate::spec::ItemSpec;
use crate::{ListedItem, Progress, PublishReport, SteamStatus, WorkshopError};

pub struct Off;

impl Steam for Off {
    fn status(&self, _app_id: u32) -> SteamStatus {
        SteamStatus::Disabled
    }

    fn list(&self, _app_id: u32) -> Result<Vec<ListedItem>, WorkshopError> {
        Err(WorkshopError::Disabled)
    }

    fn upload(
        &self,
        _spec: &ItemSpec,
        _progress: &mut dyn FnMut(&Progress),
    ) -> Result<PublishReport, WorkshopError> {
        Err(WorkshopError::Disabled)
    }
}
