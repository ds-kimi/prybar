//! StudioMDL Data: reference, LOD and animation files.

mod anim;
pub(crate) mod names;
mod phys;
pub(crate) mod phys_math;
mod phys_piece;
pub(crate) mod skeleton;
pub(crate) mod triangles;

pub use anim::{anim_names, anim_smd};
use names::{stem, unique};
pub use phys::physics_smd;
use prybar_mdl::ir::{BodyPart, SubModel};
use prybar_mdl::ModelIr;
use std::collections::HashSet;

/// One file to write: a path relative to the output folder and its text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmdFile {
    pub name: String,
    pub text: String,
}

/// Every reference and LOD SMD of a model with loaded geometry, then one
/// SMD per decoded animation; names never collide. Blank sub-models
/// (studiomdl's "blank" bodygroup entries) have no SMD.
pub fn mesh_files(ir: &ModelIr) -> Vec<SmdFile> {
    let mut taken = HashSet::new();
    let mut out: Vec<SmdFile> = mesh_entries(ir, &mut taken)
        .into_iter()
        .map(|(model, lod, name)| SmdFile {
            name,
            text: mesh_smd(ir, model, lod),
        })
        .collect();
    out.extend(anim::anim_files(ir, &mut taken));
    out
}

/// `(sub-model, lod, file name)` of every mesh SMD, in [`mesh_files`]
/// order, so companion files (VTA) can reuse the exact reference name.
pub fn mesh_entries<'a>(
    ir: &'a ModelIr,
    taken: &mut HashSet<String>,
) -> Vec<(&'a SubModel, usize, String)> {
    let mut out = Vec::new();
    for part in &ir.body_parts {
        for model in part.models.iter().filter(|m| !m.lods.is_empty()) {
            for lod in 0..model.lods.len() {
                out.push((model, lod, unique(taken, file_name(part, model, lod))));
            }
        }
    }
    out
}

pub fn mesh_smd(ir: &ModelIr, model: &SubModel, lod: usize) -> String {
    let mut s = String::from("version 1\n");
    skeleton::write(&mut s, &ir.bones);
    triangles::write(&mut s, ir, model, lod);
    s
}

fn file_name(part: &BodyPart, model: &SubModel, lod: usize) -> String {
    let raw = if model.name.is_empty() {
        &part.name
    } else {
        &model.name
    };
    let base = stem(raw, "model");
    match lod {
        0 => format!("{base}.smd"),
        n => format!("{base}_lod{n}.smd"),
    }
}
