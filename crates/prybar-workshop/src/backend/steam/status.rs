use steamworks::AppId;

use super::wait::init;
use crate::{Avatar, SteamStatus};

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

/// The logged-in user's 64x64 RGBA avatar. Steam loads it lazily, so
/// pump callbacks for up to a second before giving up.
pub fn avatar(app_id: u32) -> Option<Avatar> {
    let client = init(app_id).ok()?;
    let me = client.friends().get_friend(client.user().steam_id());
    for _ in 0..20 {
        client.run_callbacks();
        if let Some(rgba) = me.medium_avatar() {
            return Avatar::new(64, 64, rgba);
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    None
}
