//! Root bones of compiled animations are stored in the engine's animation
//! space: studiomdl turned them 90 degrees about Z and moved the piecewise
//! motion (`LX`/`LY` extraction) out of the root into `mstudiomovement_t`.
//! Undoing both gives the frames an artist's SMD would contain.

use super::rotate::{quat_mul, rotate_z, z_quat};
use crate::ir::{Movement, Pose, Vec3};

/// Source `Studio_AnimPosition`: motion accumulated up to `frame`, as a
/// position and a yaw in degrees.
pub fn movement_at(movements: &[Movement], frame: usize) -> (Vec3, f32) {
    let (mut pos, mut yaw, mut prev) = ([0.0; 3], 0.0, 0.0);
    let frame = frame as f32;
    for m in movements {
        let end = m.end_frame as f32;
        if frame <= end {
            let span = end - prev;
            let f = if span > 0.0 {
                (frame - prev) / span
            } else {
                0.0
            };
            let d = m.v0 * f + 0.5 * (m.v1 - m.v0) * f * f;
            pos = [0, 1, 2].map(|i| pos[i] + d * m.vector[i]);
            return (pos, yaw * (1.0 - f) + m.angle * f);
        }
        (pos, yaw, prev) = (m.position, m.angle, end);
    }
    (pos, yaw)
}

/// Converts one root-bone pose from animation space to SMD space.
pub fn root_to_smd(pose: Pose, motion: (Vec3, f32)) -> Pose {
    let ((offset, yaw), turn) = (motion, -90f32.to_radians());
    let yaw = yaw.to_radians();
    let moved = rotate_z(pose.position, yaw);
    let position = rotate_z([0, 1, 2].map(|i| moved[i] + offset[i]), turn);
    let quat = quat_mul(z_quat(turn), quat_mul(z_quat(yaw), pose.quat));
    Pose { position, quat }
}

/// Bind pose of a root bone expressed in animation space, for frames that
/// carry no data for it (all-zero anims, bones missing from the stream),
/// so that every frame of an anim lives in the same space.
pub fn root_rest(position: Vec3, quat: crate::ir::Quat) -> Pose {
    let turn = 90f32.to_radians();
    Pose {
        position: rotate_z(position, turn),
        quat: quat_mul(z_quat(turn), quat),
    }
}
