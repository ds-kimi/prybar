//! Zoom and pan of a flat image, with the same glide as the 3D view.

use glam::Vec2;

/// Fraction of speed kept after one second, as in the 3D camera.
const KEEP_PER_SEC: f32 = 0.002;
const REST: f32 = 4.0;
/// From a tiny overview to a texel the size of a thumbnail.
const MIN_SCALE: f32 = 0.02;
const MAX_SCALE: f32 = 64.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pan {
    /// Screen pixels per texel; `None` fits the image to the area.
    pub scale: Option<f32>,
    /// Image center relative to the area center, in screen pixels.
    pub offset: Vec2,
    /// Glide after a flick, pixels per second.
    pub slide: Vec2,
}

impl Default for Pan {
    fn default() -> Self {
        Self {
            scale: None,
            offset: Vec2::ZERO,
            slide: Vec2::ZERO,
        }
    }
}

impl Pan {
    /// The scale in use: fitted (never enlarged past 1:1 by fitting) or chosen.
    pub fn scale(&self, area: Vec2, image: Vec2) -> f32 {
        let fit = (area / image.max(Vec2::ONE)).min_element().min(1.0);
        self.scale.unwrap_or(fit).clamp(MIN_SCALE, MAX_SCALE)
    }

    /// Zooms by `factor` keeping the texel under `at` (relative to the
    /// area center) where it is.
    pub fn zoom_at(&mut self, at: Vec2, factor: f32, area: Vec2, image: Vec2) {
        let before = self.scale(area, image);
        let after = (before * factor).clamp(MIN_SCALE, MAX_SCALE);
        let k = after / before;
        if k.is_finite() {
            self.offset = at - (at - self.offset) * k;
            self.scale = Some(after);
        }
    }

    /// Advances the glide by `dt` seconds; true while still moving.
    pub fn step(&mut self, dt: f32) -> bool {
        let dt = dt.clamp(0.0, 0.1);
        self.offset += self.slide * dt;
        self.slide *= KEEP_PER_SEC.powf(dt);
        if self.slide.length() < REST || !self.slide.is_finite() {
            self.slide = Vec2::ZERO;
        }
        self.slide != Vec2::ZERO
    }
}
