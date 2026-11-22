use super::names::{stem, unique};
use super::skeleton::{bone_line, nodes};
use super::SmdFile;
use prybar_mdl::anim::math::quat_to_euler;
use prybar_mdl::anim::{movement_at, root_to_smd};
use prybar_mdl::ir::Animation;
use prybar_mdl::ModelIr;
use std::collections::HashSet;
use std::fmt::Write;

/// One SMD per animation with decoded frames. Animations whose frames sit
/// in a missing `.ani` file have none and are skipped. Delta animations
/// are written as stored (offsets from the identity), which is what
/// studiomdl reads back for a `delta` sequence. Root bones go back to SMD
/// space (see [`root_to_smd`]).
pub fn anim_files(ir: &ModelIr, taken: &mut HashSet<String>) -> Vec<SmdFile> {
    anim_names(ir, taken)
        .into_iter()
        .map(|(i, name)| SmdFile {
            name,
            text: anim_smd(ir, &ir.animations[i]),
        })
        .collect()
}

/// `(animation index, file name)` in [`anim_files`] order, so the QC can
/// point at the exact files without rendering them.
pub fn anim_names(ir: &ModelIr, taken: &mut HashSet<String>) -> Vec<(usize, String)> {
    let decoded = ir.animations.iter().enumerate();
    decoded
        .filter(|(_, a)| !a.frames.is_empty())
        .map(|(i, a)| (i, unique(taken, format!("{}.smd", stem(&a.name, "anim")))))
        .collect()
}

pub fn anim_smd(ir: &ModelIr, anim: &Animation) -> String {
    let mut s = String::from("version 1\n");
    nodes(&mut s, &ir.bones);
    s.push_str("skeleton\n");
    for (t, frame) in anim.frames.iter().enumerate() {
        let _ = writeln!(s, "time {t}");
        if ir.bones.is_empty() {
            bone_line(&mut s, 0, [0.0; 3], [0.0; 3]);
        }
        let motion = movement_at(&anim.movements, t);
        for (i, (pose, bone)) in frame.iter().zip(&ir.bones).enumerate() {
            let pose = if bone.parent < 0 {
                root_to_smd(*pose, motion)
            } else {
                *pose
            };
            bone_line(&mut s, i, pose.position, quat_to_euler(pose.quat));
        }
    }
    s.push_str("end\n");
    s
}
