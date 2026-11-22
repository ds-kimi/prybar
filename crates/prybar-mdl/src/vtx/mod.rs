//! VTX (optimized model, version 7): per-LOD triangle lists that index
//! into the VVD vertices of each mesh.

mod strip;
mod tree;

use crate::read::{table, Source};
use crate::Result;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VtxLod {
    pub switch_point: f32,
    /// One list per MDL mesh; indices are mesh-relative VVD vertex ids.
    pub meshes: Vec<Vec<[u16; 3]>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vtx {
    pub checksum: i32,
    /// `[body part][model][lod]`, mirroring the MDL body part tree.
    pub body_parts: Vec<Vec<Vec<VtxLod>>>,
}

pub fn parse_vtx(data: &[u8]) -> Result<Vtx> {
    let r = Source::new(data);
    let checksum = r.read(16)?;
    let parts = table(&r, 0, r.read(28)?, r.read(32)?, 8)?;
    let body_parts = parts
        .into_iter()
        .map(|at| tree::body_part(&r, at))
        .collect::<Result<_>>()?;
    Ok(Vtx {
        checksum,
        body_parts,
    })
}
