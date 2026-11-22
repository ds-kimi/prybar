use prybar_export::smd::{anim_smd, mesh_files};
use prybar_mdl::ir::{Animation, Bone, Pose};
use prybar_mdl::ModelIr;

fn model() -> ModelIr {
    let bone = |name: &str, parent| Bone {
        name: name.into(),
        parent,
        quat: [0.0, 0.0, 0.0, 1.0],
        ..Bone::default()
    };
    let pose = |x| Pose {
        position: [x, 0.0, 0.0],
        quat: [0.0, 0.0, 0.0, 1.0],
    };
    ModelIr {
        bones: vec![bone("root", -1), bone("child", 0)],
        animations: vec![Animation {
            name: "@run/fast".into(),
            frame_count: 2,
            frames: vec![vec![pose(1.0), pose(5.0)], vec![pose(2.0), pose(5.0)]],
            ..Animation::default()
        }],
        ..ModelIr::default()
    }
}

/// Root bones leave animation space (turned 90 degrees about Z by
/// studiomdl) while children are written as stored.
#[test]
fn writes_frames_with_root_back_in_smd_space() {
    let ir = model();
    let text = anim_smd(&ir, &ir.animations[0]);
    assert!(text.starts_with(
        "version 1\nnodes\n  0 \"root\" -1\n  1 \"child\" 0\nend\nskeleton\ntime 0\n"
    ));
    assert!(text.contains("time 0\n  0 0.000000 -1.000000 0.000000 0.000000 0.000000 -1.570796\n"));
    assert!(text.contains("  1 5.000000 0.000000 0.000000 0.000000 0.000000 0.000000\ntime 1\n"));
    assert!(text.contains("time 1\n  0 0.000000 -2.000000"));
    assert!(text.ends_with("end\n"));
}

#[test]
fn anims_get_safe_unique_names_and_skip_missing_frames() {
    let mut ir = model();
    let mut copy = ir.animations[0].clone();
    copy.name = "fast".into();
    ir.animations.push(copy);
    ir.animations.push(Animation {
        name: "in_ani_file".into(),
        frame_count: 4,
        ..Animation::default()
    });
    let names: Vec<_> = mesh_files(&ir).into_iter().map(|f| f.name).collect();
    assert_eq!(names, ["fast.smd", "fast_2.smd"]);
}
