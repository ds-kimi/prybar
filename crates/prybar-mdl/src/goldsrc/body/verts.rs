use super::Ctx;
use crate::goldsrc::transform::apply;
use crate::ir::{Vec3, Vertex};
use crate::read::{offset, table, vec3};
use crate::{Error, Result};
use prybar_io::Cursor;

/// A point or normal moved to model space, with the bone it follows.
pub type Placed = (Vec3, u8);

/// Reads `count` vectors and their one-byte bone indices; `w` is 1 for
/// points and 0 for normals. An unknown bone leaves the vector as is.
pub fn points(ctx: &Ctx, count: i32, info: i32, data: i32, w: f32) -> Result<Vec<Placed>> {
    let rows = table(ctx.r, 0, count, data, 12)?;
    let bones = ctx.r.bytes(offset(0, info)?, rows.len())?;
    rows.into_iter()
        .zip(bones)
        .map(|(at, &bone)| {
            let v = vec3(&mut Cursor::new(ctx.r.reader, at))?;
            let placed = ctx
                .mats
                .get(usize::from(bone))
                .map_or(v, |m| apply(m, v, w));
            Ok((placed, bone))
        })
        .collect()
}

/// One triangle corner: `[vertex, normal, s, t]` as GoldSrc stores it.
pub fn vertex(pts: (&[Placed], &[Placed]), corner: [i16; 4], dims: (f32, f32)) -> Result<Vertex> {
    let pick = |list: &[Placed], i: i16| {
        usize::try_from(i)
            .ok()
            .and_then(|i| list.get(i).copied())
            .ok_or(Error::Invalid {
                what: "triangle vertex",
                value: i.into(),
            })
    };
    let (position, bone) = pick(pts.0, corner[0])?;
    let (normal, _) = pick(pts.1, corner[1])?;
    Ok(Vertex {
        position,
        normal,
        uv: [f32::from(corner[2]) / dims.0, f32::from(corner[3]) / dims.1],
        weights: vec![(bone, 1.0)],
    })
}
