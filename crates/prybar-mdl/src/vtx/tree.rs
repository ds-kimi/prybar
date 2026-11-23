use super::{shape::int, shape::Shape, strip::mesh_triangles, VtxLod};
use crate::read::{table, Source};
use crate::Result;

pub fn body_part(r: &Source, at: usize, sh: &Shape) -> Result<Vec<Vec<VtxLod>>> {
    table(r, at, r.read(at)?, r.read(at + 4)?, 8)?
        .into_iter()
        .map(|m| model(r, m, sh))
        .collect()
}

fn model(r: &Source, at: usize, sh: &Shape) -> Result<Vec<VtxLod>> {
    table(r, at, r.read(at)?, r.read(at + 4)?, 12)?
        .into_iter()
        .map(|l| lod(r, l, sh))
        .collect()
}

fn lod(r: &Source, at: usize, sh: &Shape) -> Result<VtxLod> {
    let meshes = table(r, at, r.read(at)?, r.read(at + 4)?, sh.mesh_stride)?
        .into_iter()
        .map(|m| {
            let groups = (int(r, m, sh.narrow)?, r.read(m + 4)?);
            mesh_triangles(r, m, groups, sh)
        })
        .collect::<Result<_>>()?;
    Ok(VtxLod {
        switch_point: r.read(at + 8)?,
        meshes,
    })
}
