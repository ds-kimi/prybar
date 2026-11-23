//! The direction tables MDL 2531 packed vertices index into. The engine
//! ships them as constants; both are rings of unit vectors at equal polar
//! steps from -Z to +Z, so they are rebuilt from that rule instead of
//! being embedded.

use crate::ir::Vec3;
use std::sync::OnceLock;

/// Unskinned vertices address the fine table by byte offset.
const FINE_STRIDE: usize = 12;

/// Direction for an unskinned vertex's normal byte offset.
pub fn fine(byte_offset: u16) -> Option<Vec3> {
    static T: OnceLock<Vec<Vec3>> = OnceLock::new();
    let at = usize::from(byte_offset);
    if at % FINE_STRIDE != 0 {
        return None;
    }
    T.get_or_init(|| build(64)).get(at / FINE_STRIDE).copied()
}

/// Direction for a compressed vertex's normal index.
pub fn coarse(index: u8) -> Option<Vec3> {
    static T: OnceLock<Vec<Vec3>> = OnceLock::new();
    T.get_or_init(|| build(12)).get(usize::from(index)).copied()
}

/// Ring k of n holds 4 * floor(n / 2 * sin(k * pi / n) + 0.9) azimuth
/// steps, one at each pole: 5314 directions for n = 64, 202 for n = 12.
fn build(rings: u32) -> Vec<Vec3> {
    let mut out = Vec::new();
    for ring in 0..=rings {
        let polar = f64::from(ring) * std::f64::consts::PI / f64::from(rings);
        let z = -polar.cos();
        if ring == 0 || ring == rings {
            // Exact poles: sin(pi) is not quite 0 in floating point.
            out.push([0.0, 0.0, z as f32]);
            continue;
        }
        let sin = polar.sin();
        let steps = 4 * (f64::from(rings) / 2.0 * sin + 0.9).floor() as u32;
        for step in 0..steps {
            let az = f64::from(step) * std::f64::consts::TAU / f64::from(steps);
            out.push([(sin * az.cos()) as f32, (sin * az.sin()) as f32, z as f32]);
        }
    }
    out
}
