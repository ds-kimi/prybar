//! The read-only Steam questions, asked off the UI thread, for the
//! Workshop game the user picked.

use std::sync::Arc;

use gpui::RenderImage;
use prybar_service::tools::WorkshopGames;
use prybar_workshop::{Avatar, ListedItem, SteamStatus};

use super::Backend;

pub type Answer = (
    SteamStatus,
    Result<Vec<ListedItem>, String>,
    Option<Arc<RenderImage>>,
);

/// Installed games first: the page follows the pick while that game is
/// still installed, else the first one found. With none installed Steam
/// is not asked at all; the page shows how to add one instead.
pub fn ask(
    b: Backend,
    chosen: Option<u32>,
    want_avatar: bool,
) -> (WorkshopGames, Option<u32>, Answer) {
    let games = (b.targets)();
    let found = |a: &u32| games.detected.iter().any(|g| g.app_id == *a);
    let app = chosen
        .filter(found)
        .or(games.detected.first().map(|g| g.app_id));
    let Some(app) = app else {
        return (games, None, (SteamStatus::NotOwned, Ok(Vec::new()), None));
    };
    let s = (b.status)(app);
    let ready = matches!(s, SteamStatus::Ready { .. });
    let list = if ready {
        (b.items)(app)
    } else {
        Ok(Vec::new())
    };
    let face = (ready && want_avatar).then(|| (b.avatar)(app)).flatten();
    (games, Some(app), (s, list, face.and_then(picture)))
}

/// RGBA from Steam into the BGRA GPUI paints.
fn picture(a: Avatar) -> Option<Arc<RenderImage>> {
    let mut px = a.rgba;
    for p in px.as_chunks_mut::<4>().0 {
        p.swap(0, 2);
    }
    let buf = image::RgbaImage::from_raw(a.width, a.height, px)?;
    Some(Arc::new(RenderImage::new(vec![image::Frame::new(buf)])))
}
