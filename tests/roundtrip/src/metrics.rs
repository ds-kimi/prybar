use prybar_mdl::ir::{Lod, SubModel};
use prybar_mdl::ModelIr;
use serde::Serialize;
use std::collections::HashSet;

/// What a decompile must preserve for studiomdl to rebuild the same
/// model. Positions and weights are left to the SMD tests: studiomdl
/// re-welds and re-sorts vertices, so only counts survive a rebuild.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Metrics {
    pub bones: Vec<String>,
    pub vertices: usize,
    /// `(name, frames of the first animation)` per sequence.
    pub sequences: Vec<(String, usize)>,
    pub flexes: usize,
    pub materials: Vec<String>,
}

impl Metrics {
    pub fn of(ir: &ModelIr) -> Self {
        let vertices = ir
            .body_parts
            .iter()
            .flat_map(|b| &b.models)
            .map(referenced)
            .sum();
        let frames = |anims: &[i16]| {
            let first = anims.first().and_then(|&a| usize::try_from(a).ok());
            first
                .and_then(|a| ir.animations.get(a))
                .map_or(0, |a| a.frame_count)
        };
        let materials = used_materials(ir);
        Self {
            bones: ir
                .bones
                .iter()
                .map(|b| b.name.to_ascii_lowercase())
                .collect(),
            vertices,
            sequences: ir
                .sequences
                .iter()
                .map(|s| (s.name.to_ascii_lowercase(), frames(&s.anims)))
                .collect(),
            flexes: ir.flex_descs.len(),
            materials,
        }
    }
}

/// Only materials a mesh points at: shipped models keep stale entries
/// in the table that a fresh studiomdl run drops. Lowercased and with
/// `/` because studiomdl may change both.
fn used_materials(ir: &ModelIr) -> Vec<String> {
    let mut used: Vec<String> = ir
        .body_parts
        .iter()
        .flat_map(|b| &b.models)
        .flat_map(|m| &m.meshes)
        .filter_map(|m| usize::try_from(m.material).ok())
        .filter_map(|i| ir.materials.get(i))
        .map(|m| m.name.to_ascii_lowercase().replace('\\', "/"))
        .collect();
    used.sort();
    used.dedup();
    used
}

/// Vertices each LOD draws, summed over LODs. The raw VVD count is not
/// comparable: old studiomdl builds left unreferenced vertices (12 of 354
/// in `props_c17/lampfixture01a`) and kept separate LOD copies of
/// vertices a current build shares (`eli`: same count per LOD, 7501 vs
/// 6851 over all LODs).
fn referenced(m: &SubModel) -> usize {
    if m.lods.is_empty() {
        return m.vertices.len();
    }
    let per_lod = |l: &Lod| {
        let tris = l.meshes.iter().flat_map(|t| &t.triangles);
        tris.flatten().collect::<HashSet<_>>().len()
    };
    m.lods.iter().map(per_lod).sum()
}
