//! Velocity left over after a drag, decayed per second so the glide is
//! the same at 60 and 120 fps.

use glam::{Vec2, Vec3};

use super::Orbit;

/// Fraction of speed kept after one second: about a third of a second
/// of glide that is still noticeable.
const KEEP_PER_SEC: f32 = 0.002;
/// Below this many pixels per second the glide stops for good.
const REST: f32 = 4.0;
/// Zoom eases toward its goal at this rate (1/s).
const ZOOM_RATE: f32 = 18.0;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Inertia {
    /// Pixels per second of orbit and of pan.
    pub spin: Vec2,
    pub slide: Vec2,
    /// Distance the eased zoom is heading to.
    pub zoom_to: Option<f32>,
    /// World point the zoom closes in on: the one under the cursor.
    pub toward: Option<Vec3>,
}

impl Inertia {
    /// Velocity from the drag samples of the last ~80 ms: older ones
    /// say nothing about the flick at release.
    pub fn release(samples: &[(f32, Vec2)], now: f32) -> Vec2 {
        let recent: Vec<_> = samples.iter().filter(|(t, _)| now - t < 0.08).collect();
        let (Some(a), Some(b)) = (recent.first(), recent.last()) else {
            return Vec2::ZERO;
        };
        let dt = b.0 - a.0;
        if dt < 1e-3 {
            return Vec2::ZERO;
        }
        let v = (b.1 - a.1) / dt;
        if v.is_finite() {
            v
        } else {
            Vec2::ZERO
        }
    }

    /// Advances the camera by `dt` seconds; true while still moving.
    pub fn step(&mut self, cam: &mut Orbit, dt: f32, h: f32) -> bool {
        let dt = dt.clamp(0.0, 0.1);
        cam.rotate(self.spin * dt);
        cam.pan(self.slide * dt, h);
        let keep = KEEP_PER_SEC.powf(dt);
        self.spin *= keep;
        self.slide *= keep;
        for v in [&mut self.spin, &mut self.slide] {
            if v.length() < REST {
                *v = Vec2::ZERO;
            }
        }
        if let Some(goal) = self.zoom_to {
            let k = 1.0 - (-ZOOM_RATE * dt).exp();
            let before = cam.distance;
            cam.zoom(1.0 + (goal / before - 1.0) * k, self.toward);
            // A goal past the zoom limits is never reached: stop when
            // the limit holds the distance still.
            let still = (cam.distance - before).abs() < goal * 1e-4;
            if still || (cam.distance - goal).abs() < goal * 1e-3 {
                self.zoom_to = None;
                self.toward = None;
            }
        }
        self.moving()
    }

    pub fn moving(&self) -> bool {
        self.spin != Vec2::ZERO || self.slide != Vec2::ZERO || self.zoom_to.is_some()
    }
}
