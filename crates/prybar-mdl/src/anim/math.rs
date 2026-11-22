//! Rotation conversions with Source's conventions: a `RadianEuler` is
//! (roll about X, pitch about Y, yaw about Z), the order SMD files use.

use crate::ir::{Quat, Vec3};

/// Source `AngleQuaternion(RadianEuler, Quaternion)`.
pub fn euler_to_quat(e: Vec3) -> Quat {
    let (sr, cr) = (e[0] * 0.5).sin_cos();
    let (sp, cp) = (e[1] * 0.5).sin_cos();
    let (sy, cy) = (e[2] * 0.5).sin_cos();
    let (sr_cp, cr_sp) = (sr * cp, cr * sp);
    let (cr_cp, sr_sp) = (cr * cp, sr * sp);
    [
        sr_cp * cy - cr_sp * sy,
        cr_sp * cy + sr_cp * sy,
        cr_cp * sy - sr_sp * cy,
        cr_cp * cy + sr_sp * sy,
    ]
}

/// Source `QuaternionAngles`: through the rotation matrix, so it matches
/// the angles studiomdl stores next to each bone quaternion.
pub fn quat_to_euler(q: Quat) -> Vec3 {
    let [x, y, z, w] = q;
    let forward = [
        1.0 - 2.0 * (y * y + z * z),
        2.0 * (x * y + w * z),
        2.0 * (x * z - w * y),
    ];
    let left = [
        2.0 * (x * y - w * z),
        1.0 - 2.0 * (x * x + z * z),
        2.0 * (y * z + w * x),
    ];
    let up_z = 1.0 - 2.0 * (x * x + y * y);
    let xy = (forward[0] * forward[0] + forward[1] * forward[1]).sqrt();
    let pitch = (-forward[2]).atan2(xy);
    if xy > 0.001 {
        [left[2].atan2(up_z), pitch, forward[1].atan2(forward[0])]
    } else {
        // Gimbal lock: roll and yaw share one axis, Source folds it into yaw.
        [0.0, pitch, (-left[0]).atan2(left[1])]
    }
}

/// Source's float16 has no inf/NaN: exponent 31 saturates to the largest
/// finite value, which keeps forged data from poisoning the output.
pub fn half_to_f32(h: u16) -> f32 {
    let sign = if h & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exp = i32::from((h >> 10) & 0x1f);
    let mant = f32::from(h & 0x3ff);
    let mag = match exp {
        0 => mant * 2f32.powi(-24),
        31 => 65504.0,
        _ => (1.0 + mant / 1024.0) * 2f32.powi(exp - 15),
    };
    sign * mag
}
