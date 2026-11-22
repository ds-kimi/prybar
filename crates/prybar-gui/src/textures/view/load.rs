//! Background work: finding the textures, then decoding the one shown.

use std::path::PathBuf;
use std::sync::Arc;

use gpui::{AppContext as _, Context, RenderImage, Task};

use super::{Channel, Shown, Status, TextureView};
use crate::textures::TextureSet;

/// Parses the model for its material names, then looks every texture up.
pub fn start(path: PathBuf, cx: &mut Context<TextureView>) -> Task<()> {
    let job = cx.background_spawn(async move {
        let ir = prybar_service::model::open(&path.to_string_lossy()).map_err(|e| e.to_string())?;
        Ok::<_, String>(crate::textures::load(&path, &ir))
    });
    cx.spawn(async move |this, cx| {
        let set = job.await;
        let _ = this.update(cx, |v, cx| {
            match set {
                Ok(set) => (v.set, v.status) = (Some(set), Status::Ready),
                Err(e) => v.status = Status::Failed(e),
            }
            v.select(0, cx)
        });
    })
}

impl TextureView {
    /// Decodes the selected slot at the chosen mip and channel; the old
    /// image stays until the new one lands so switching never flashes.
    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        let Some(set) = self.set.clone() else {
            return cx.notify();
        };
        let slot = set.slots.get(self.selected);
        self.problem = slot.and_then(|s| s.error.clone());
        let Some(base) = slot
            .filter(|s| s.info.is_some())
            .and_then(|s| s.report.base_texture.clone())
        else {
            self.drops.extend(self.shown.take().map(|s| s.image));
            return cx.notify();
        };
        let (mip, channel) = (self.mip, self.channel);
        let job = cx.background_spawn(async move { decode(&set, &base, mip, channel) });
        self.decode = Some(cx.spawn(async move |this, cx| {
            let shown = job.await;
            let _ = this.update(cx, |v, cx| {
                match shown {
                    Ok(s) => v.drops.extend(v.shown.replace(s).map(|s| s.image)),
                    Err(e) => v.problem = Some(e),
                }
                cx.notify()
            });
        }));
        cx.notify();
    }
}

fn decode(set: &TextureSet, base: &str, mip: u8, channel: Channel) -> Result<Shown, String> {
    let tex =
        prybar_service::texture::texture(&set.sources, base, mip).map_err(|e| e.to_string())?;
    let (width, height) = (tex.image.width, tex.image.height);
    let image = image(tex.image, channel).ok_or("Texture size does not match its data")?;
    Ok(Shown {
        image,
        width,
        height,
    })
}

/// RGBA into the BGRA GPUI paints. RGB shows the color opaque; alpha
/// shows the alpha channel as grey, where cutouts and masks live.
pub fn image(img: prybar_vtf::Image, channel: Channel) -> Option<Arc<RenderImage>> {
    let mut px = img.rgba;
    for p in px.as_chunks_mut::<4>().0 {
        *p = match channel {
            Channel::Rgb => [p[2], p[1], p[0], 255],
            Channel::Alpha => [p[3], p[3], p[3], 255],
        };
    }
    let buf = image::RgbaImage::from_raw(img.width, img.height, px)?;
    Some(Arc::new(RenderImage::new(vec![image::Frame::new(buf)])))
}
