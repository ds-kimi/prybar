use super::{Eyeball, Flex, Lod, Vec3, Vertex};
use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct BodyPart {
    pub name: String,
    /// Multiplier that turns a model index into a body value, kept so the
    /// QC writer can reproduce `$bodygroup` order exactly.
    pub base: i32,
    pub models: Vec<SubModel>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct SubModel {
    pub name: String,
    pub kind: i32,
    pub bounding_radius: f32,
    pub vertex_count: i32,
    /// Byte offset into the VVD vertex block, needed to slice its vertices.
    pub vertex_index: i32,
    pub tangents_index: i32,
    pub eyeball_count: i32,
    pub eyeballs: Vec<Eyeball>,
    pub meshes: Vec<Mesh>,
    /// Filled from the VVD; empty until geometry is loaded.
    #[serde(skip)]
    pub vertices: Vec<Vertex>,
    /// Filled from the VTX; empty until geometry is loaded.
    #[serde(skip)]
    pub lods: Vec<Lod>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Mesh {
    pub material: i32,
    pub vertex_count: i32,
    /// First vertex of this mesh, relative to its model.
    pub vertex_offset: i32,
    pub flex_count: i32,
    pub id: i32,
    pub center: Vec3,
    pub lod_vertex_counts: [i32; 8],
    pub flexes: Vec<Flex>,
}
