use crate::anim::math::half_to_f32;
use crate::ir::{Flex, VertexDelta};
use crate::read::Source;
use crate::read::{table, Words};
use crate::Result;

/// `mstudioflex_t` size.
const FLEX_STRIDE: usize = 60;
/// `STUDIO_VERT_ANIM_WRINKLE` adds one short to each vertex record.
const WRINKLE: u8 = 1;

/// How the delta shorts of `mstudiovertanim_t` turn into floats.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeltaCodec {
    /// float16, what PC builds of studiomdl write.
    Half,
    /// Fixed point times the header scale.
    Fixed(f32),
}

/// Flexes of one `mstudiomesh_t`; the index is relative to the mesh.
pub fn mesh_flexes(
    r: &Source,
    mesh_at: usize,
    count: i32,
    index: i32,
    codec: DeltaCodec,
) -> Result<Vec<Flex>> {
    table(r, mesh_at, count, index, FLEX_STRIDE)?
        .into_iter()
        .map(|at| flex(r, at, codec))
        .collect()
}

fn flex(r: &Source, at: usize, codec: DeltaCodec) -> Result<Flex> {
    let w = Words::read(r, at, FLEX_STRIDE / 4)?;
    let wrinkle = w.i(8) & 0xff == i32::from(WRINKLE);
    let stride = if wrinkle { 18 } else { 16 };
    let deltas = table(r, at, w.i(5), w.i(6), stride)?
        .into_iter()
        .map(|v| delta(r, v, wrinkle, codec))
        .collect::<Result<_>>()?;
    Ok(Flex {
        desc: w.i(0),
        partner: w.i(7),
        targets: [w.f(1), w.f(2), w.f(3), w.f(4)],
        wrinkle,
        deltas,
    })
}

fn delta(r: &Source, at: usize, wrinkle: bool, codec: DeltaCodec) -> Result<VertexDelta> {
    let shorts = if wrinkle { 7 } else { 6 };
    let raw: Vec<u16> = r.array(at + 4, shorts, 7)?;
    let f = |n: usize| match codec {
        DeltaCodec::Half => half_to_f32(raw[n]),
        DeltaCodec::Fixed(scale) => f32::from(raw[n] as i16) * scale,
    };
    // The wrinkle short is always fixed point, even in float16 builds.
    let scale = match codec {
        DeltaCodec::Half => 1.0 / 4096.0,
        DeltaCodec::Fixed(s) => s,
    };
    Ok(VertexDelta {
        index: r.read(at)?,
        speed: r.read(at + 2)?,
        side: r.read(at + 3)?,
        position: [f(0), f(1), f(2)],
        normal: [f(3), f(4), f(5)],
        wrinkle: if wrinkle {
            f32::from(raw[6] as i16) * scale
        } else {
            0.0
        },
    })
}
