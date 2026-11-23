//! VTX (optimized model, version 7, and 107 for MDL 2531): per-LOD
//! triangle lists that index into the vertices of each mesh.

mod shape;
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
    // 107 is the only other version known; anything else reads as 7, as before.
    let sh = if r.read::<i32>(0)? == 107 {
        shape::V107
    } else {
        shape::V7
    };
    let checksum = r.read(16)?;
    let parts = table(&r, 0, r.read(28)?, r.read(32)?, 8)?;
    let body_parts = parts
        .into_iter()
        .map(|at| tree::body_part(&r, at, &sh))
        .collect::<Result<_>>()?;
    Ok(Vtx {
        checksum,
        body_parts,
    })
}
