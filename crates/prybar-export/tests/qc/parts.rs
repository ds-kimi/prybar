use prybar_mdl::ir::*;

pub fn bone(name: &str, parent: i32, x: f32) -> Bone {
    Bone {
        name: name.into(),
        parent,
        position: [x, 0.0, 0.0],
        rotation: [0.0, std::f32::consts::FRAC_PI_2, 0.0],
        quat: [0.0, 0.0, 0.0, 1.0],
        pose_to_bone: [
            [1.0, 0.0, 0.0, -x],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ],
        ..Bone::default()
    }
}

pub fn sub(name: &str, lods: usize, flexes: Vec<Flex>) -> SubModel {
    SubModel {
        name: name.into(),
        meshes: vec![Mesh {
            vertex_count: 1,
            flexes,
            ..Mesh::default()
        }],
        vertices: vec![Vertex::default()],
        lods: (0..lods)
            .map(|i| Lod {
                switch_point: i as f32 * 10.0,
                ..Lod::default()
            })
            .collect(),
        ..SubModel::default()
    }
}

/// A face with a stereo pair, a mono flex and one eyeball.
pub fn face() -> SubModel {
    let flex = |desc, partner| Flex {
        desc,
        partner,
        targets: [0.0, 1.0, 1.0, 1.0],
        ..Flex::default()
    };
    let mut face = sub("face", 1, vec![flex(0, 1), flex(2, 0)]);
    face.eyeballs = vec![Eyeball {
        name: "eye_right".into(),
        bone: 1,
        origin: [1.0, 2.0, 3.0],
        z_offset: 0.0699,
        radius: 0.5,
        texture: 1,
        iris_scale: 1.0,
        ..Eyeball::default()
    }];
    face
}

/// Root, a bone-merged head with its own surface prop, a jiggle tail.
pub fn bones() -> Vec<Bone> {
    let mut jiggle = bone("tail", 1, 3.0);
    let mut params = vec![0.0; 34];
    params[..2].copy_from_slice(&[5.0, 0.5]);
    params[9..11].copy_from_slice(&[-0.5, 0.5]);
    params[30] = 0.5;
    jiggle.procedural = Some(ProcBone::Jiggle(Jiggle {
        flags: 0x01 | 0x04 | 0x80,
        params,
    }));
    let mut head = bone("head", 0, 1.0);
    head.flags = 0x0004_0000;
    head.surface_prop = "flesh".into();
    vec![bone("root", -1, 0.0), head, jiggle]
}
