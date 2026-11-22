use super::bone_name;
use prybar_mdl::anim::math::quat_to_euler;
use prybar_mdl::ir::{AimAt, QuatTrigger, Vec3};
use prybar_mdl::ModelIr;
use std::fmt::Write;

/// `<helper> bone parent control_parent control`, then one `<trigger>` per
/// pose: tolerance, trigger angles, target angles (degrees), target pos.
pub fn helper(s: &mut String, ir: &ModelIr, bone: usize, control: i32, triggers: &[QuatTrigger]) {
    let parent = |i: i32| {
        usize::try_from(i)
            .ok()
            .and_then(|i| ir.bones.get(i))
            .map_or(-1, |b| b.parent)
    };
    let me = i32::try_from(bone).unwrap_or(-1);
    let _ = writeln!(
        s,
        "\n<helper> {} {} {} {}\n<basepos> 0 0 0",
        bone_name(ir, me),
        bone_name(ir, parent(me)),
        bone_name(ir, parent(control)),
        bone_name(ir, control)
    );
    for t in triggers {
        let _ = write!(s, "<trigger> {:.6}", t.tolerance.to_degrees());
        for v in [quat_to_euler(t.trigger), quat_to_euler(t.quat)] {
            vec(s, v.map(f32::to_degrees));
        }
        vec(s, t.position);
        s.push('\n');
    }
}

pub fn aim(s: &mut String, ir: &ModelIr, bone: usize, aim: &AimAt, target: &str) {
    let _ = writeln!(
        s,
        "\n<aimconstraint> {} {} {target}",
        bone_name(ir, i32::try_from(bone).unwrap_or(-1)),
        bone_name(ir, aim.parent)
    );
    for (tag, v) in [
        ("aimvector", aim.aim_vector),
        ("upvector", aim.up_vector),
        ("basepos", aim.base_pos),
    ] {
        let _ = write!(s, "<{tag}>");
        vec(s, v);
        s.push('\n');
    }
}

/// `-0.000000` from float noise would make diffs noisy.
fn vec(s: &mut String, v: Vec3) {
    let [x, y, z] = v.map(|c| if c.abs() < 5e-7 { 0.0 } else { c });
    let _ = write!(s, " {x:.6} {y:.6} {z:.6}");
}
