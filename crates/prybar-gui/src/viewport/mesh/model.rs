use prybar_mdl::ir::{ModelIr, SubModel};

use super::{Part, Scene, MAX_TRIANGLES};

/// Appends LOD 0 of one sub-model, one part per mesh.
pub fn add(scene: &mut Scene, ir: &ModelIr, model: &SubModel, skin: usize) {
    let Some(lod) = model.lods.first() else {
        return;
    };
    let base = push_vertices(scene, model);
    let count = model.vertices.len();
    for (mi, lm) in lod.meshes.iter().enumerate() {
        let first = scene.indices.len() as u32;
        for t in &lm.triangles {
            if t.iter().all(|&i| (i as usize) < count) && scene.triangles() < MAX_TRIANGLES {
                scene.indices.extend(t.iter().map(|&i| base + i));
            }
        }
        let count = scene.indices.len() as u32 - first;
        if count > 0 {
            let material = material(ir, model, mi, skin);
            scene.parts.push(Part {
                first,
                count,
                material,
            });
        }
    }
}

/// Returns the index of the first vertex pushed.
fn push_vertices(scene: &mut Scene, model: &SubModel) -> u32 {
    let base = (scene.vertices.len() / super::STRIDE) as u32;
    for v in &model.vertices {
        scene.vertices.extend(v.position.iter().chain(&v.normal));
        scene.vertices.extend(v.uv);
    }
    base
}

/// The mesh's material slot, remapped by the skin family when it has one.
fn material(ir: &ModelIr, model: &SubModel, mesh: usize, skin: usize) -> usize {
    let slot = model
        .meshes
        .get(mesh)
        .map_or(0, |m| m.material.max(0) as usize);
    ir.skins
        .get(skin)
        .and_then(|row| row.get(slot))
        .map_or(slot, |&m| usize::from(m))
}
