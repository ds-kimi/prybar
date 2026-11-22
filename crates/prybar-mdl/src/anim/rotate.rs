//! Small rotation helpers about the Z (yaw) axis.

use crate::ir::{Quat, Vec3};

pub fn rotate_z(v: Vec3, angle: f32) -> Vec3 {
    let (s, c) = angle.sin_cos();
    [v[0] * c - v[1] * s, v[0] * s + v[1] * c, v[2]]
}

pub fn quat_mul(a: Quat, b: Quat) -> Quat {
    let [ax, ay, az, aw] = a;
    let [bx, by, bz, bw] = b;
    [
        aw * bx + ax * bw + ay * bz - az * by,
        aw * by - ax * bz + ay * bw + az * bx,
        aw * bz + ax * by - ay * bx + az * bw,
        aw * bw - ax * bx - ay * by - az * bz,
    ]
}

pub fn z_quat(angle: f32) -> Quat {
    let (s, c) = (angle * 0.5).sin_cos();
    [0.0, 0.0, s, c]
}
