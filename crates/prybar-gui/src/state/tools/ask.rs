//! The read-only Steam questions, asked off the UI thread.

use std::sync::Arc;

use gpui::RenderImage;
use prybar_workshop::spec::GMOD_APP_ID;
use prybar_workshop::{Avatar, ListedItem, SteamStatus};

use super::Backend;

pub type Answer = (
    SteamStatus,
    Result<Vec<ListedItem>, String>,
    Option<Arc<RenderImage>>,
);

/// Status first: the list and the picture only make sense when ready.
pub fn ask(b: Backend, want_avatar: bool) -> Answer {
    let s = (b.status)(GMOD_APP_ID);
    let ready = matches!(s, SteamStatus::Ready { .. });
    let list = if ready {
        (b.items)(GMOD_APP_ID)
    } else {
        Ok(Vec::new())
    };
    let face = (ready && want_avatar)
        .then(|| (b.avatar)(GMOD_APP_ID))
        .flatten();
    (s, list, face.and_then(picture))
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
