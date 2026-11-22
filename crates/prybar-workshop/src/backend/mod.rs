mod avatar;
pub mod mock;
#[cfg(not(feature = "workshop"))]
mod off;
#[cfg(feature = "workshop")]
mod steam;

pub use avatar::avatar;

use crate::spec::ItemSpec;
use crate::{ListedItem, Progress, PublishReport, SteamStatus, WorkshopError};

/// Every call that reaches Steam. `upload` gets a spec whose content and
/// preview are already staged (a folder holding the GMA, a fitted image).
pub trait Steam: Send + Sync {
    fn status(&self, app_id: u32) -> SteamStatus;
    fn list(&self, app_id: u32) -> Result<Vec<ListedItem>, WorkshopError>;
    fn upload(
        &self,
        spec: &ItemSpec,
        progress: &mut dyn FnMut(&Progress),
    ) -> Result<PublishReport, WorkshopError>;
}

/// The real backend, or the one that says the feature is off.
pub fn steam() -> &'static dyn Steam {
    #[cfg(feature = "workshop")]
    return &steam::SteamClient;
    #[cfg(not(feature = "workshop"))]
    return &off::Off;
}
