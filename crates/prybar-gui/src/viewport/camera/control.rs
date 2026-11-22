//! Direct manipulation of the orbit, all in screen pixels so mouse and
//! trackpad feel the same at any window size.

use glam::{Vec2, Vec3};

use super::{Orbit, PITCH_LIMIT};

/// Radians per pixel of drag: half a turn across a 600 px viewport.
const TURN: f32 = std::f32::consts::PI / 600.0;
/// Closest and farthest zoom, in model radii, so the model never fills
/// the near plane and never shrinks to a dot.
const NEAR: f32 = 0.35;
const FAR: f32 = 12.0;

impl Orbit {
    pub fn rotate(&mut self, d: Vec2) {
        self.yaw = (self.yaw - d.x * TURN).rem_euclid(std::f32::consts::TAU);
        self.pitch = (self.pitch + d.y * TURN).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    }

    /// Moves the target so the point under the cursor follows it; `h` is
    /// the viewport height in the same pixels as `d`.
    pub fn pan(&mut self, d: Vec2, h: f32) {
        let per_px = 2.0 * self.distance * (self.fov_y * 0.5).tan() / h.max(1.0);
        let fwd = (self.target - self.eye()).normalize_or_zero();
        let right = fwd.cross(Vec3::Z).normalize_or_zero();
        let up = right.cross(fwd);
        self.target += (-right * d.x + up * d.y) * per_px;
        self.limit();
    }

    /// `factor` < 1 moves closer. `toward` is the world point under the
    /// cursor: the target slides toward it in step with the zoom.
    pub fn zoom(&mut self, factor: f32, toward: Option<Vec3>) {
        let factor = if factor.is_finite() { factor } else { 1.0 };
        let old = self.distance;
        let r = self.radius.max(1e-3);
        self.distance = (old * factor).clamp(r * NEAR, r * FAR);
        if let Some(p) = toward.filter(|p| p.is_finite()) {
            self.target += (p - self.target) * (1.0 - self.distance / old);
        }
        self.limit();
    }

    /// Keeps the target within a few radii of the model so it can never
    /// drift off into empty space.
    fn limit(&mut self) {
        let max = self.radius.max(1e-3) * 3.0;
        let off = self.target - self.home;
        if off.length() > max {
            self.target = self.home + off.normalize() * max;
        }
    }
}
