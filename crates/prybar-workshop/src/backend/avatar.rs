//! The account picture, read-only. Kept out of the [`super::Steam`]
//! trait: it only decorates the UI and never takes part in a publish.

use crate::Avatar;

#[cfg(feature = "workshop")]
pub fn avatar(app_id: u32) -> Option<Avatar> {
    super::steam::status::avatar(app_id)
}

/// Builds without Steam have no account to show.
#[cfg(not(feature = "workshop"))]
pub fn avatar(_app_id: u32) -> Option<Avatar> {
    None
}
