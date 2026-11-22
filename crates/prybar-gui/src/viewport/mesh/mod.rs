//! Turns a parsed model into flat GPU-ready buffers: the chosen LOD of the
//! chosen body group models, materials remapped by the chosen skin.

mod bones;
mod bounds;
#[cfg(test)]
mod fixtures;
mod model;
#[cfg(test)]
mod tests;

use prybar_mdl::ir::ModelIr;

pub use bones::bones;
pub use bounds::Bounds;
pub use model::lods;

/// Floats per vertex: position, normal, uv.
pub const STRIDE: usize = 8;
/// A hostile file can claim millions of triangles; past this the preview
/// would stall the GPU upload for no visual gain.
pub(crate) const MAX_TRIANGLES: usize = 4_000_000;

/// One draw range sharing a material.
#[derive(Debug, Clone, PartialEq)]
pub struct Part {
    pub first: u32,
    pub count: u32,
    pub material: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Scene {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub parts: Vec<Part>,
    pub bounds: Bounds,
    /// Bone segments (parent, child) in bind pose.
    pub bones: Vec<[[f32; 3]; 2]>,
    /// Up to three `(bone, weight)` per vertex, for posing on the CPU.
    pub weights: Vec<[(u8, f32); 3]>,
}

/// Which variant of the model to show.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pick {
    /// `body[i]` is the sub-model of body part `i` (0 when missing).
    pub body: Vec<usize>,
    pub skin: usize,
    /// Falls back to LOD 0 when a sub-model has fewer levels.
    pub lod: usize,
}

impl Scene {
    pub fn triangles(&self) -> usize {
        self.indices.len() / 3
    }
}

pub fn build(ir: &ModelIr, pick: &Pick) -> Result<Scene, String> {
    let mut scene = Scene::default();
    for (pi, part) in ir.body_parts.iter().enumerate() {
        let at = pick.body.get(pi).copied().unwrap_or(0);
        if let Some(model) = part.models.get(at).or(part.models.first()) {
            model::add(&mut scene, ir, model, pick);
        }
    }
    if scene.indices.is_empty() {
        return Err("This model has no meshes to show".into());
    }
    scene.bounds = Bounds::of(&scene.vertices);
    scene.bones = bones(ir);
    Ok(scene)
}
