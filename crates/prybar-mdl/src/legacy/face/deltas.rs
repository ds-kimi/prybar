//! MDL 2531 mesh flexes. A vertex animation is either full (float
//! position delta) or packed (direction index plus length byte); normal
//! deltas are always packed. Directions index the fine normal table.

use super::super::geometry::normals;
use crate::ir::{Flex, Vec3, VertexDelta};
use crate::read::{table, Source};
use crate::{Error, Result};

/// `mstudioflex_t` of 2531: desc, four targets, i16 count, offset, i16 kind.
const FLEX_STRIDE: usize = 32;
/// Packed lengths are bytes over these ranges, in model units.
const POSITION_RANGE: f32 = 8.0;
const NORMAL_RANGE: f32 = 2.0;

/// Flexes of one mesh; the index is relative to the mesh.
pub fn mesh_flexes(r: &Source, mesh_at: usize, count: i32, index: i32) -> Result<Vec<Flex>> {
    table(r, mesh_at, count, index, FLEX_STRIDE)?
        .into_iter()
        .map(|at| flex(r, at))
        .collect()
}

fn flex(r: &Source, at: usize) -> Result<Flex> {
    let count: i16 = r.read(at + 20)?;
    let kind: i16 = r.read(at + 28)?;
    let stride = match kind {
        0 => 20,
        1 => 8,
        _ => {
            return Err(Error::Invalid {
                what: "MDL 2531 flex vertex animation type",
                value: i64::from(kind),
            })
        }
    };
    let t: Vec<f32> = r.array(at + 4, 4, 4)?;
    let deltas = table(r, at, count.into(), r.read(at + 24)?, stride)?
        .into_iter()
        .map(|v| delta(r, v, kind == 0))
        .collect::<Result<_>>()?;
    Ok(Flex {
        desc: r.read(at)?,
        partner: 0,
        targets: [t[0], t[1], t[2], t[3]],
        wrinkle: false,
        deltas,
    })
}

/// Full: index, pad, three floats, normal u16 + u8, pad. Packed: index,
/// position u16, normal u16, position u8, normal u8.
fn delta(r: &Source, at: usize, full: bool) -> Result<VertexDelta> {
    let index: u16 = r.read(at)?;
    let (position, normal_at, normal_len_at) = if full {
        let p = [r.read(at + 4)?, r.read(at + 8)?, r.read(at + 12)?];
        (p, at + 16, at + 18)
    } else {
        let len: u8 = r.read(at + 6)?;
        (
            scaled(r.read(at + 2)?, len, POSITION_RANGE)?,
            at + 4,
            at + 7,
        )
    };
    Ok(VertexDelta {
        index,
        speed: 255,
        side: 0,
        position,
        normal: scaled(r.read(normal_at)?, r.read(normal_len_at)?, NORMAL_RANGE)?,
        wrinkle: 0.0,
    })
}

fn scaled(direction: u16, length: u8, range: f32) -> Result<Vec3> {
    let d = normals::fine(direction).ok_or(Error::Invalid {
        what: "MDL 2531 flex delta direction",
        value: i64::from(direction),
    })?;
    let len = f32::from(length) / 255.0 * range;
    Ok(d.map(|c| c * len))
}
