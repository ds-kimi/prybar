use steamworks::AppId;

use super::wait::init;
use crate::SteamStatus;

/// Read-only checks in the order a user fixes them.
pub fn status(app_id: u32) -> SteamStatus {
    let Ok(client) = init(app_id) else {
        return SteamStatus::NotRunning;
    };
    if !client.user().logged_on() {
        return SteamStatus::NotLoggedIn;
    }
    if !client.apps().is_subscribed_app(AppId(app_id)) {
        return SteamStatus::NotOwned;
    }
    SteamStatus::Ready {
        user: client.friends().name(),
    }
}
