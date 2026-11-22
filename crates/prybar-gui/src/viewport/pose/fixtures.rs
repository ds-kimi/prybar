use prybar_mdl::ir::{Animation, Bone, ModelIr, Pose, Sequence};

const STILL: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// Root at the origin, child one unit up; the anim slides the child
/// from 1 to 3 over two frames at 10 fps, looping.
pub fn rig(parent: i32) -> ModelIr {
    let frame = |z| {
        vec![
            root(),
            Pose {
                position: [0.0, 0.0, z],
                quat: STILL,
            },
        ]
    };
    let anim = Animation {
        fps: 10.0,
        frames: vec![frame(1.0), frame(3.0)],
        ..Animation::default()
    };
    let seq = Sequence {
        anims: vec![0],
        flags: 1,
        ..Sequence::default()
    };
    ModelIr {
        bones: vec![bone(-1, 0.0), bone(parent, 1.0)],
        animations: vec![anim],
        sequences: vec![seq],
        ..ModelIr::default()
    }
}

/// Root frames are in animation space (turned 90 degrees about Z), so
/// the inverse turn lands the root on its bind pose.
fn root() -> Pose {
    let s = std::f32::consts::FRAC_1_SQRT_2;
    Pose {
        position: [0.0; 3],
        quat: [0.0, 0.0, s, s],
    }
}

fn bone(parent: i32, z: f32) -> Bone {
    Bone {
        parent,
        position: [0.0, 0.0, z],
        quat: STILL,
        pose_to_bone: [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., -z]],
        ..Bone::default()
    }
}
