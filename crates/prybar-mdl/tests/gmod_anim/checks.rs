use prybar_mdl::anim::math::{euler_to_quat, quat_to_euler};
use prybar_mdl::anim::{movement_at, root_to_smd};
use prybar_mdl::ir::{Animation, Bone, Pose};
use prybar_mdl::ModelIr;

fn same_pose(pose: &Pose, b: &Bone) -> (bool, bool) {
    let dot: f32 = pose.quat.iter().zip(b.quat).map(|(x, y)| x * y).sum();
    let near = pose
        .position
        .iter()
        .zip(b.position)
        .all(|(x, y)| (x - y).abs() <= 1e-2);
    (dot.abs() > 0.9999, near)
}

/// The bind-pose euler angles are stored next to each quaternion, so the
/// conversions used for every anim frame can be checked against studiomdl.
pub fn check_bind_euler(stem: &str, ir: &ModelIr) {
    for b in &ir.bones {
        let e = quat_to_euler(b.quat);
        for q in [euler_to_quat(b.rotation), euler_to_quat(e)] {
            let dot: f32 = q.iter().zip(b.quat).map(|(x, y)| x * y).sum();
            assert!(
                dot.abs() > 0.9999,
                "{stem} {}: {e:?} vs {:?}",
                b.name,
                b.rotation
            );
        }
    }
}

/// Frame 0 vs bind pose, root bones taken back to SMD space or skipped.
/// Returns `(rotations match, positions match)`.
pub fn frame0_vs_bind(ir: &ModelIr, a: &Animation, roots: bool) -> (bool, bool) {
    let (mut rot, mut pos) = (true, true);
    let pairs = ir.bones.iter().zip(&a.frames[0]);
    for (b, pose) in pairs.filter(|(b, _)| roots || b.parent >= 0) {
        let pose = if b.parent < 0 {
            root_to_smd(*pose, movement_at(&a.movements, 0))
        } else {
            *pose
        };
        let (r, p) = same_pose(&pose, b);
        (rot, pos) = (rot && r, pos && p);
    }
    (rot, pos)
}

pub fn find<'a>(ir: &'a ModelIr, name: &str) -> Option<&'a Animation> {
    ir.animations
        .iter()
        .find(|a| a.name.eq_ignore_ascii_case(name))
}
