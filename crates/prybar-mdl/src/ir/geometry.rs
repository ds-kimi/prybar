use super::Vec3;
use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    /// Texture coordinates as stored by Source (v grows downwards).
    pub uv: [f32; 2],
    /// `(bone index, weight)`, at most three entries.
    pub weights: Vec<(u8, f32)>,
}

/// One level of detail of a sub-model.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Lod {
    pub switch_point: f32,
    /// Parallel to `SubModel::meshes`.
    pub meshes: Vec<LodMesh>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct LodMesh {
    /// Indices into `SubModel::vertices`, wound the way SMD expects.
    pub triangles: Vec<[u32; 3]>,
}
