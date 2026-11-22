//! Body parts, their models and the triangle-command meshes.

mod mesh;
mod tris;
mod verts;

use super::header::{fixed_name, GoldHeader, BODY_PARTS};
use crate::ir::{BodyPart, Lod, Mat3x4, SubModel};
use crate::read::{table, Source, Words};
use crate::Result;

/// What a mesh needs beyond its own bytes.
pub struct Ctx<'a, 'b> {
    pub r: &'a Source<'b>,
    /// Model-space matrix per bone.
    pub mats: &'a [Mat3x4],
    /// Texture size per skin slot, to turn texel coordinates into UVs.
    pub dims: &'a [(f32, f32)],
}

/// `mstudiobodyparts_t`: 64-byte name, model count, base, model index.
pub fn body_parts(ctx: &Ctx, h: &GoldHeader) -> Result<Vec<BodyPart>> {
    let (count, index) = h.span(BODY_PARTS);
    table(ctx.r, 0, count, index, 76)?
        .into_iter()
        .map(|at| {
            let w = Words::read(ctx.r, at + 64, 3)?;
            let models = table(ctx.r, 0, w.i(0), w.i(2), 112)?
                .into_iter()
                .map(|m| model(ctx, m))
                .collect::<Result<_>>()?;
            Ok(BodyPart {
                name: fixed_name(ctx.r, at, 64)?,
                base: w.i(1),
                models,
            })
        })
        .collect()
}

/// `mstudiomodel_t`: 64-byte name then 12 words.
fn model(ctx: &Ctx, at: usize) -> Result<SubModel> {
    let w = Words::read(ctx.r, at + 64, 12)?;
    let points = verts::points(ctx, w.i(4), w.i(5), w.i(6), 1.0)?;
    let normals = verts::points(ctx, w.i(7), w.i(8), w.i(9), 0.0)?;
    let mut sub = SubModel {
        name: fixed_name(ctx.r, at, 64)?,
        kind: w.i(0),
        bounding_radius: w.f(1),
        vertex_count: w.i(4),
        lods: vec![Lod::default()],
        ..SubModel::default()
    };
    for mesh_at in table(ctx.r, 0, w.i(2), w.i(3), 20)? {
        mesh::mesh(ctx, mesh_at, (&points, &normals), &mut sub)?;
    }
    Ok(sub)
}
