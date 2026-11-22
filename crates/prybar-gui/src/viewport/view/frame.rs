use std::sync::Arc;

use gpui::{point, px, size, Bounds, Pixels, RenderImage};

use crate::viewport::render::Frame;

/// Frames are drawn at device size, so they map 1:1 to pixels. A frame
/// from before a resize keeps its own size, centered, until the new one
/// lands: no stretch and no blur, only a moment of margin.
pub fn native(b: Bounds<Pixels>, img: &RenderImage, scale: f32) -> Bounds<Pixels> {
    let d = img.size(0);
    let (w, h) = (px(d.width.0 as f32 / scale), px(d.height.0 as f32 / scale));
    let origin = point(
        b.origin.x + (b.size.width - w) / 2.0,
        b.origin.y + (b.size.height - h) / 2.0,
    );
    Bounds::new(origin, size(w, h))
}

/// BGRA bytes into a GPUI image; `None` when the sizes disagree.
pub fn image(f: Frame) -> Option<Arc<RenderImage>> {
    let buf = image::RgbaImage::from_raw(f.width, f.height, f.bgra)?;
    Some(Arc::new(RenderImage::new(vec![image::Frame::new(buf)])))
}
