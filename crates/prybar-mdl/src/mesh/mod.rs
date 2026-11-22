//! Joins MDL, VVD and VTX into per-model vertices and LOD triangle lists.

mod lod;

use crate::ir::{SubModel, Vertex};
use crate::vtx::{Vtx, VtxLod};
use crate::vvd::Vvd;
use crate::{Error, ModelIr, Result};

/// studiomdl stamps the same checksum into all three files; a mismatch
/// means the companions belong to another build of the model.
pub fn load_geometry(ir: &mut ModelIr, vvd: &Vvd, vtx: &Vtx) -> Result<()> {
    for sum in [vvd.checksum, vtx.checksum] {
        if sum != ir.checksum {
            return Err(Error::Invalid {
                what: "companion file checksum",
                value: sum.into(),
            });
        }
    }
    // Sub-models may alias one vertex range; cap the copies so a hostile
    // MDL cannot multiply the VVD pool many times over.
    let mut budget = vvd.vertices.len().saturating_mul(4).max(1 << 16);
    for (pi, part) in ir.body_parts.iter_mut().enumerate() {
        for (mi, model) in part.models.iter_mut().enumerate() {
            model.vertices = model_vertices(model, &vvd.vertices, &mut budget)?;
            let lods = vtx.body_parts.get(pi).and_then(|p| p.get(mi));
            model.lods = lods
                .map(|l| l.iter().map(|l: &VtxLod| lod::build(model, l)).collect())
                .transpose()?
                .unwrap_or_default();
        }
    }
    Ok(())
}

/// `vertex_index` is a byte offset into the VVD vertex block.
fn model_vertices(model: &SubModel, pool: &[Vertex], budget: &mut usize) -> Result<Vec<Vertex>> {
    let bad = Error::Invalid {
        what: "model vertex range starting at",
        value: model.vertex_index.into(),
    };
    let start = usize::try_from(model.vertex_index).map_err(|_| bad.clone())? / 48;
    let count = usize::try_from(model.vertex_count).map_err(|_| bad.clone())?;
    let end = start.checked_add(count).ok_or(bad.clone())?;
    *budget = budget.checked_sub(count).ok_or(bad.clone())?;
    Ok(pool.get(start..end).ok_or(bad)?.to_vec())
}
