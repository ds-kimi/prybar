use super::{strip::mesh_triangles, VtxLod};
use crate::read::{table, Source};
use crate::Result;

/// `MeshHeader_t` is packed: two ints and a flags byte.
const MESH_STRIDE: usize = 9;

pub fn body_part(r: &Source, at: usize) -> Result<Vec<Vec<VtxLod>>> {
    table(r, at, r.read(at)?, r.read(at + 4)?, 8)?
        .into_iter()
        .map(|m| model(r, m))
        .collect()
}

fn model(r: &Source, at: usize) -> Result<Vec<VtxLod>> {
    table(r, at, r.read(at)?, r.read(at + 4)?, 12)?
        .into_iter()
        .map(|l| lod(r, l))
        .collect()
}

fn lod(r: &Source, at: usize) -> Result<VtxLod> {
    let meshes = table(r, at, r.read(at)?, r.read(at + 4)?, MESH_STRIDE)?
        .into_iter()
        .map(|m| mesh_triangles(r, m))
        .collect::<Result<_>>()?;
    Ok(VtxLod {
        switch_point: r.read(at + 8)?,
        meshes,
    })
}
