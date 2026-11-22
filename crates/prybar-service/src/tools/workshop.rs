use std::path::Path;

use prybar_workshop::addon::AddonCheck;
use prybar_workshop::SteamStatus;

use super::publish::workshop_error;
use crate::Result;

/// Read-only: changes nothing in Steam.
pub fn steam_status(app_id: u32) -> SteamStatus {
    prybar_workshop::status(app_id)
}

/// Every `addon.json` and whitelist problem of an addon folder.
pub fn addon_check(dir: &Path) -> Result<AddonCheck> {
    prybar_workshop::addon::check_addon(dir).map_err(workshop_error)
}
