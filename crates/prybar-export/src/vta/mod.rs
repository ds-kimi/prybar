//! Vertex animation (VTA) files: the flex shapes of one sub-model as
//! absolute vertex positions per frame, frame 0 being the base mesh.

pub(crate) mod frames;

use crate::smd::{mesh_entries, skeleton::nodes, SmdFile};
use prybar_mdl::ir::{SubModel, Vec3};
use prybar_mdl::ModelIr;
use std::collections::HashSet;
use std::fmt::Write;

/// One VTA per sub-model with flexes, named after its reference SMD so a
/// QC `flexfile` line can point at both.
pub fn vta_files(ir: &ModelIr) -> Vec<SmdFile> {
    mesh_entries(ir, &mut HashSet::new())
        .into_iter()
        .filter(|(m, lod, _)| *lod == 0 && m.meshes.iter().any(|x| !x.flexes.is_empty()))
        .map(|(model, _, name)| SmdFile {
            name: format!("{}.vta", name.trim_end_matches(".smd")),
            text: vta(ir, model),
        })
        .collect()
}

pub fn vta(ir: &ModelIr, model: &SubModel) -> String {
    let frames = frames::frames(ir, model);
    let mut s = String::from("version 1\n");
    nodes(&mut s, &ir.bones);
    s.push_str("skeleton\ntime 0 # basis shape key\n");
    for (i, f) in frames.iter().enumerate() {
        let _ = writeln!(s, "time {} # {}", i + 1, f.label);
    }
    s.push_str("end\nvertexanimation\ntime 0 # basis shape key\n");
    for (i, v) in model.vertices.iter().enumerate() {
        line(&mut s, i, v.position, v.normal);
    }
    for (i, f) in frames.iter().enumerate() {
        let _ = writeln!(s, "time {} # {}", i + 1, f.label);
        frame(&mut s, model, f);
    }
    s.push_str("end\n");
    s
}

/// Only moved vertices are listed, as absolute positions.
fn frame(s: &mut String, model: &SubModel, f: &frames::Frame) {
    let add = |a: Vec3, b: Vec3| [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
    for &(base, flex) in &f.parts {
        for d in &flex.deltas {
            let at = base + usize::from(d.index);
            if let Some(v) = model.vertices.get(at) {
                line(s, at, add(v.position, d.position), add(v.normal, d.normal));
            }
        }
    }
}

fn line(s: &mut String, i: usize, [px, py, pz]: Vec3, [nx, ny, nz]: Vec3) {
    let _ = writeln!(s, "  {i} {px:.6} {py:.6} {pz:.6} {nx:.6} {ny:.6} {nz:.6}");
}
