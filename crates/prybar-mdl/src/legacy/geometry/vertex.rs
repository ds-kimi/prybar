//! The three MDL 2531 vertex formats: skinned (float, up to four bones),
//! unskinned (u16 quantized) and compressed (u8 quantized). Both packed
//! formats are rigid to bone 0 and scale into the model's bounds.

use super::normals;
use crate::ir::{Vec3, Vertex};
use crate::read::{vec3, Source};
use crate::{Error, Result};
use prybar_io::Cursor;

/// Per-model dequantization for the packed formats.
pub struct Frame {
    pub offset: Vec3,
    pub scale: Vec3,
}

/// 44 bytes: three weight bytes, an influence selector, four i16 bones,
/// then position, normal and uv as floats.
pub fn skinned(r: &Source, at: usize) -> Result<Vertex> {
    let raw = r.bytes(at, 4)?;
    let count = usize::from(raw[3] % 5).max(1);
    // Weights sum to 255; the fourth is implied by the other three.
    let fourth = 255u8
        .saturating_sub(raw[0])
        .saturating_sub(raw[1])
        .saturating_sub(raw[2]);
    let stored = [raw[0], raw[1], raw[2], fourth];
    let mut weights = Vec::with_capacity(count);
    for (i, &w) in stored.iter().enumerate().take(count) {
        let bone = r.read::<i16>(at + 4 + i * 2)?;
        if let (true, Ok(b)) = (w > 0, u8::try_from(bone)) {
            weights.push((b, f32::from(w) / 255.0));
        }
    }
    if weights.is_empty() {
        weights.push((0, 1.0));
    }
    let mut c = Cursor::new(r.reader, at + 12);
    Ok(Vertex {
        position: vec3(&mut c)?,
        normal: vec3(&mut c)?,
        uv: [c.read()?, c.read()?],
        weights,
    })
}

/// Unskinned (12 bytes) when `wide`, compressed (8 bytes) otherwise.
pub fn packed(r: &Source, at: usize, frame: &Frame, wide: bool) -> Result<Vertex> {
    let (q, max, normal, uv_at) = if wide {
        let q: [u16; 3] = [r.read(at)?, r.read(at + 2)?, r.read(at + 4)?];
        let off: u16 = r.read(at + 6)?;
        (
            q.map(f32::from),
            1.0,
            normals::fine(off).ok_or(off.into()),
            8,
        )
    } else {
        let b = r.bytes(at, 4)?;
        let q = [b[0], b[1], b[2]].map(f32::from);
        (q, 255.0, normals::coarse(b[3]).ok_or(b[3].into()), 4)
    };
    let normal = normal.map_err(|value| Error::Invalid {
        what: "MDL 2531 vertex normal",
        value,
    })?;
    let u: u16 = r.read(at + uv_at)?;
    let v: u16 = r.read(at + uv_at + 2)?;
    Ok(Vertex {
        position: std::array::from_fn(|i| q[i] / max * frame.scale[i] + frame.offset[i]),
        normal,
        uv: [f32::from(u) / 65535.0, f32::from(v) / 65535.0],
        weights: vec![(0, 1.0)],
    })
}
