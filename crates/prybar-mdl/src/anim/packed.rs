//! Fixed-size compressed rotations and positions (`Quaternion48`,
//! `Quaternion64`, `Vector48`).

use super::math::half_to_f32;
use crate::ir::{Quat, Vec3};
use crate::Result;
use prybar_io::Reader;

pub fn quat48(r: &Reader, at: usize) -> Result<Quat> {
    let x: u16 = r.read(at)?;
    let y: u16 = r.read(at + 2)?;
    let zw: u16 = r.read(at + 4)?;
    let x = (f32::from(x) - 32768.0) / 32768.0;
    let y = (f32::from(y) - 32768.0) / 32768.0;
    let z = (f32::from(zw & 0x7fff) - 16384.0) / 16384.0;
    Ok(with_w(x, y, z, zw & 0x8000 != 0))
}

pub fn quat64(r: &Reader, at: usize) -> Result<Quat> {
    let bits: u64 = r.read(at)?;
    let axis = |shift: u32| ((bits >> shift) & 0x1f_ffff) as f32;
    let x = (axis(0) - 1_048_576.0) / 1_048_576.5;
    let y = (axis(21) - 1_048_576.0) / 1_048_576.5;
    let z = (axis(42) - 1_048_576.0) / 1_048_576.5;
    Ok(with_w(x, y, z, bits >> 63 != 0))
}

pub fn vector48(r: &Reader, at: usize) -> Result<Vec3> {
    let h = |i: usize| -> Result<f32> { Ok(half_to_f32(r.read(at + i * 2)?)) };
    Ok([h(0)?, h(1)?, h(2)?])
}

/// w is implied by unit length; clamping guards against forged xyz whose
/// squares sum past 1.
fn with_w(x: f32, y: f32, z: f32, negative: bool) -> Quat {
    let w = (1.0 - x * x - y * y - z * z).max(0.0).sqrt();
    [x, y, z, if negative { -w } else { w }]
}
