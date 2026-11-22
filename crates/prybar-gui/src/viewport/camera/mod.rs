//! Orbit camera around the model, Source axes (Z up).

#[cfg(test)]
mod tests;

use glam::{Mat4, Vec3};

use super::mesh::Bounds;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Orbit {
    pub target: Vec3,
    /// Radians around Z.
    pub yaw: f32,
    /// Radians above the ground plane, kept off the poles.
    pub pitch: f32,
    pub distance: f32,
    /// Model radius, so clip planes follow the model and not the zoom.
    pub radius: f32,
    pub fov_y: f32,
}

/// Checked on HL2 characters: their face looks down -X in model space,
/// so a three-quarter front view sits on that side.
const START_YAW: f32 = std::f32::consts::PI + 0.6;
const START_PITCH: f32 = 0.3;
const PITCH_LIMIT: f32 = 1.55;

impl Orbit {
    /// Frames the whole box with a small margin at any aspect >= 1.
    pub fn fit(b: &Bounds) -> Self {
        let fov_y = 40f32.to_radians();
        let radius = b.radius();
        Self {
            target: Vec3::from(b.center()),
            yaw: START_YAW,
            pitch: START_PITCH,
            distance: radius / (fov_y * 0.5).sin() * 1.05,
            radius,
            fov_y,
        }
    }

    pub fn eye(&self) -> Vec3 {
        let pitch = self.pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);
        let (sy, cy) = self.yaw.sin_cos();
        let (sp, cp) = pitch.sin_cos();
        self.target + Vec3::new(cy * cp, sy * cp, sp) * self.distance
    }

    /// Projection times view, depth 0..1 as wgpu expects. `aspect` is
    /// width / height of the target in pixels; a narrow viewport widens
    /// the vertical field so the model never gets cropped sideways.
    pub fn view_proj(&self, aspect: f32) -> Mat4 {
        let aspect = if aspect.is_finite() && aspect > 0.01 {
            aspect
        } else {
            1.0
        };
        let fov = if aspect < 1.0 {
            2.0 * ((self.fov_y * 0.5).tan() / aspect).atan()
        } else {
            self.fov_y
        };
        let near = (self.distance - self.radius * 2.0)
            .max(self.radius * 0.01)
            .max(1e-3);
        let far = self.distance + self.radius * 8.0;
        let view = Mat4::look_at_rh(self.eye(), self.target, Vec3::Z);
        Mat4::perspective_rh(fov, aspect, near, far) * view
    }
}
