use prybar_mdl::ir::{BodyPart, Lod, LodMesh, Mesh, ModelIr, SubModel, Vertex};

pub fn tri_model(triangles: Vec<[u32; 3]>, material: i32) -> SubModel {
    let v = |x: f32| Vertex {
        position: [x, 0.0, x * 2.0],
        ..Vertex::default()
    };
    SubModel {
        vertices: vec![v(0.0), v(1.0), v(2.0)],
        meshes: vec![Mesh {
            material,
            ..Mesh::default()
        }],
        lods: vec![Lod {
            switch_point: 0.0,
            meshes: vec![LodMesh { triangles }],
        }],
        ..SubModel::default()
    }
}

pub fn ir(models: Vec<SubModel>) -> ModelIr {
    ModelIr {
        body_parts: vec![BodyPart {
            models,
            ..BodyPart::default()
        }],
        skins: vec![vec![0, 1], vec![1, 0]],
        ..ModelIr::default()
    }
}
