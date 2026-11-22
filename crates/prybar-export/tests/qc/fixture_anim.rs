use prybar_mdl::ir::*;
use prybar_mdl::ModelIr;

/// A 2x1 blend with an event, a pose layer, a weight list and nodes.
fn walk() -> Sequence {
    let mut seq = Sequence {
        name: "walk".into(),
        activity_name: "ACT_WALK".into(),
        activity_weight: 1,
        flags: 0x1,
        group_size: [2, 1],
        anims: vec![0, 1],
        ..Sequence::default()
    };
    (seq.param_index, seq.param_start, seq.param_end) = ([0, -1], [-1.0, 0.0], [1.0, 0.0]);
    (seq.fade_in, seq.fade_out) = (0.2, 0.2);
    seq.events = vec![Event {
        cycle: 0.5,
        options: "Foot.Step".into(),
        name: "AE_FOOTSTEP".into(),
        ..Event::default()
    }];
    seq.auto_layers = vec![AutoLayer {
        sequence: 1,
        pose: 0,
        flags: 0x40 | 0x4000,
        ramp: [0.0, 0.1, 0.9, 1.0],
    }];
    seq.bone_weights = vec![1.0, 0.5, 0.0];
    (seq.entry_node, seq.exit_node) = (1, 2);
    seq.key_values = "\"faceposer\" { }".into();
    seq
}

fn gesture() -> Sequence {
    Sequence {
        name: "gesture".into(),
        flags: 0x4 | 0x80,
        group_size: [1, 1],
        anims: vec![1],
        bone_weights: vec![1.0, 0.5, 0.0],
        auto_layers: vec![AutoLayer::default()],
        ..Sequence::default()
    }
}

/// One decoded looping walk and one anim whose frames are missing.
fn animations() -> Vec<Animation> {
    let pose = Pose {
        position: [0.0; 3],
        quat: [0.0, 0.0, 0.0, 1.0],
    };
    let movement = Movement {
        end_frame: 10,
        motion_flags: 0x40 | 0x80,
        ..Movement::default()
    };
    let walk = Animation {
        name: "@walk".into(),
        fps: 30.0,
        flags: 0x1,
        frame_count: 11,
        frames: vec![vec![pose; 3]; 11],
        movements: vec![movement],
        ..Animation::default()
    };
    let missing = Animation {
        name: "a_gesture".into(),
        fps: 30.0,
        frame_count: 5,
        ..Animation::default()
    };
    vec![walk, missing]
}

pub fn anims() -> ModelIr {
    let move_x = PoseParam {
        name: "move_x".into(),
        start: -1.0,
        end: 1.0,
        ..PoseParam::default()
    };
    ModelIr {
        pose_params: vec![move_x],
        animations: animations(),
        sequences: vec![walk(), gesture()],
        ..ModelIr::default()
    }
}
