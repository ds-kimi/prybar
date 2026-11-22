//! Turns a parsed model into flat GPU-ready buffers: LOD 0 of the chosen
//! body group models, materials remapped by the chosen skin.

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
}

impl Scene {
    pub fn triangles(&self) -> usize {
        self.indices.len() / 3
    }
}

/// `body[i]` picks the sub-model of body part `i` (0 when missing).
pub fn build(ir: &ModelIr, body: &[usize], skin: usize) -> Result<Scene, String> {
    let mut scene = Scene::default();
    for (pi, part) in ir.body_parts.iter().enumerate() {
        let pick = body.get(pi).copied().unwrap_or(0);
        if let Some(model) = part.models.get(pick).or(part.models.first()) {
            model::add(&mut scene, ir, model, skin);
        }
    }
    if scene.indices.is_empty() {
        return Err("This model has no meshes to show".into());
    }
    scene.bounds = Bounds::of(&scene.vertices);
    scene.bones = bones(ir);
    Ok(scene)
}
