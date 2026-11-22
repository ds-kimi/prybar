//! The texture screen: the open model's materials, where each base
//! texture was found or looked for, and a zoomable preview of one of them.

mod about;
mod canvas;
mod facts;
mod input;
mod load;
mod paint;
mod pan;
mod side;
#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::sync::Arc;

use glam::Vec2;
use gpui::{Bounds, Context, FocusHandle, Pixels, Point, RenderImage, Task};

use super::TextureSet;
pub use pan::Pan;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Status {
    #[default]
    Idle,
    Loading,
    Ready,
    Failed(String),
}

/// What the preview shows of the texture.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Channel {
    #[default]
    Rgb,
    Alpha,
}

/// The decoded image on screen and the facts shown beside it.
pub struct Shown {
    pub image: Arc<RenderImage>,
    pub width: u32,
    pub height: u32,
}

#[derive(Default)]
pub struct TextureView {
    path: Option<PathBuf>,
    pub status: Status,
    pub set: Option<Arc<TextureSet>>,
    pub selected: usize,
    pub mip: u8,
    pub channel: Channel,
    pub pan: Pan,
    pub shown: Option<Shown>,
    /// Why the selected texture cannot be shown, when it cannot.
    pub problem: Option<String>,
    drops: Vec<Arc<RenderImage>>,
    drag: Option<canvas::Drag>,
    tick: Option<std::time::Instant>,
    focus: Option<FocusHandle>,
    /// Window position of the preview region, for cursor math.
    area: Bounds<Pixels>,
    load: Option<Task<()>>,
    /// Only the newest decode matters.
    decode: Option<Task<()>>,
}

impl TextureView {
    /// Switches to another model; the same path again does nothing.
    pub fn show(&mut self, path: Option<PathBuf>, cx: &mut Context<Self>) {
        if path == self.path {
            return;
        }
        self.path = path.clone();
        (self.set, self.shown, self.problem, self.decode) = (None, None, None, None);
        self.status = if path.is_some() {
            Status::Loading
        } else {
            Status::Idle
        };
        self.load = path.map(|p| load::start(p, cx));
        cx.notify();
    }

    /// Picks a material slot; mip and view start over for the new image.
    pub fn select(&mut self, slot: usize, cx: &mut Context<Self>) {
        (self.selected, self.mip, self.pan) = (slot, 0, Pan::default());
        self.refresh(cx);
    }

    /// Mip level, clamped to what the texture has.
    pub fn set_mip(&mut self, mip: u8, cx: &mut Context<Self>) {
        let slot = self.set.as_ref().and_then(|s| s.slots.get(self.selected));
        let mips = slot.and_then(|s| s.info.as_ref()).map_or(1, |i| i.mips);
        self.mip = mip.min(mips.saturating_sub(1));
        self.refresh(cx);
    }

    /// Zooms keeping the texel under the cursor in place.
    pub fn zoom(&mut self, at: Point<Pixels>, factor: f32) {
        let Some(shown) = &self.shown else { return };
        let b = self.area;
        let area = Vec2::new(f32::from(b.size.width), f32::from(b.size.height));
        let rel =
            Vec2::new(f32::from(at.x - b.origin.x), f32::from(at.y - b.origin.y)) - area / 2.0;
        let texels = Vec2::new(shown.width as f32, shown.height as f32);
        self.pan.zoom_at(rel, factor, area, texels);
    }
}
