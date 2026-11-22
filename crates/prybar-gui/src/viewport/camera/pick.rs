//! Screen points back into the scene, for zoom-to-cursor, double-click
//! focus and framing.

use glam::{Vec2, Vec3};

use super::Orbit;

impl Orbit {
    /// World point under `ndc` (-1..1, y up) on the plane through the
    /// target that faces the camera. No depth readback: close enough to
    /// steer zoom and focus, and it never misses into empty space.
    pub fn under(&self, ndc: Vec2, aspect: f32) -> Option<Vec3> {
        let inv = self.view_proj(aspect).inverse();
        let a = inv.project_point3(ndc.extend(0.0));
        let b = inv.project_point3(ndc.extend(1.0));
        let dir = (b - a).normalize_or_zero();
        let n = (self.target - self.eye()).normalize_or_zero();
        let denom = dir.dot(n);
        if denom.abs() < 1e-6 {
            return None;
        }
        let t = (self.target - a).dot(n) / denom;
        let p = a + dir * t;
        p.is_finite().then_some(p)
    }

    /// Back to the whole model at the fitted distance, keeping the angle
    /// the user chose.
    pub fn framed(&self, fit: &Orbit) -> Orbit {
        Orbit {
            yaw: self.yaw,
            pitch: self.pitch,
            ..*fit
        }
    }
}

#[cfg(test)]
#[test]
fn under_center_is_target_and_framed_keeps_angle() {
    let fit = Orbit::fit(&crate::viewport::mesh::Bounds {
        min: [-1.0, -2.0, 0.0],
        max: [1.0, 2.0, 4.0],
    });
    let mut cam = fit;
    let p = cam.under(Vec2::ZERO, 1.5).unwrap();
    assert!((p - cam.target).length() < 1e-3);
    cam.rotate(Vec2::new(100.0, 50.0));
    cam.zoom(0.5, Some(p + Vec3::X));
    let back = cam.framed(&fit);
    assert_eq!(
        (back.yaw, back.distance, back.target),
        (cam.yaw, fit.distance, fit.target)
    );
}
