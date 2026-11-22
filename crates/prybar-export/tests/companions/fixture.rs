use prybar_mdl::ir::*;
use prybar_mdl::ModelIr;

fn bone(name: &str, parent: i32, x: f32) -> Bone {
    // Unrotated bones: model to bone space is a shift by -x.
    let pose_to_bone = [
        [1.0, 0.0, 0.0, -x],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ];
    Bone {
        name: name.into(),
        parent,
        position: [x, 0.0, 0.0],
        quat: [0.0, 0.0, 0.0, 1.0],
        pose_to_bone,
        ..Bone::default()
    }
}

fn flex(desc: i32, x: f32) -> Flex {
    Flex {
        desc,
        targets: [0.0, 1.0, 1.0, 1.0],
        deltas: vec![VertexDelta {
            index: 1,
            position: [x, 0.0, 0.0],
            ..VertexDelta::default()
        }],
        ..Flex::default()
    }
}

/// Two meshes sharing flex `smile` (one frame) plus `blink` on mesh 1.
fn model() -> SubModel {
    let vertex = |x: f32| Vertex {
        position: [x, 0.0, 0.0],
        normal: [0.0, 0.0, 1.0],
        ..Vertex::default()
    };
    let mesh = |offset: i32, flexes: Vec<Flex>| Mesh {
        vertex_count: 2,
        vertex_offset: offset,
        flexes,
        ..Mesh::default()
    };
    SubModel {
        name: "face".into(),
        vertices: (0..4).map(|i| vertex(i as f32)).collect(),
        meshes: vec![
            mesh(0, vec![flex(0, 0.5)]),
            mesh(2, vec![flex(0, 0.25), flex(1, 2.0)]),
        ],
        lods: vec![Lod::default()],
        ..SubModel::default()
    }
}

pub fn ir() -> ModelIr {
    let mut helper = bone("ValveBiped.helper", 0, 2.0);
    helper.procedural = Some(ProcBone::QuatInterp {
        control: 1,
        triggers: vec![QuatTrigger {
            tolerance: std::f32::consts::FRAC_PI_2,
            trigger: [0.0, 0.0, 0.0, 1.0],
            position: [1.0, 2.0, 3.0],
            quat: [0.0, 0.0, 0.0, 1.0],
        }],
    });
    ModelIr {
        bones: vec![
            bone("ValveBiped.root", -1, 0.0),
            bone("ValveBiped.arm", 0, 1.0),
            helper,
        ],
        body_parts: vec![BodyPart {
            name: "body".into(),
            models: vec![model()],
            ..BodyPart::default()
        }],
        flex_descs: vec!["smile".into(), "blink".into()],
        ..ModelIr::default()
    }
}
