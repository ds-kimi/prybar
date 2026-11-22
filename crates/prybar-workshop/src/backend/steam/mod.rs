//! Real Steam calls. Not exercised by tests: they need a running,
//! logged-in Steam client that owns the app.

mod apply;
mod list;
mod status;
mod update;
mod wait;

use super::Steam;
use crate::spec::ItemSpec;
use crate::{ListedItem, Progress, PublishReport, SteamStatus, WorkshopError};

pub struct SteamClient;

impl Steam for SteamClient {
    fn status(&self, app_id: u32) -> SteamStatus {
        status::status(app_id)
    }

    fn list(&self, app_id: u32) -> Result<Vec<ListedItem>, WorkshopError> {
        list::list(app_id)
    }

    fn upload(
        &self,
        spec: &ItemSpec,
        progress: &mut dyn FnMut(&Progress),
    ) -> Result<PublishReport, WorkshopError> {
        update::publish(spec, progress)
    }
}
