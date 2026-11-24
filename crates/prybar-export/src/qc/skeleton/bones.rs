use crate::qc::fmt::{num, quote, vec3};
use prybar_mdl::ir::Bone;
use prybar_mdl::ModelIr;
use std::fmt::Write;

const BONE_USED_BY_BONE_MERGE: i32 = 0x0004_0000;

/// `$definebone` pins the exact bind pose and order, so a recompile
/// keeps bone indices that animations and `$includemodel`s rely on.
/// Rotation goes out in degrees as y z x, the order studiomdl reads.
pub fn write(s: &mut String, ir: &ModelIr) {
    for bone in &ir.bones {
        let [rx, ry, rz] = bone.rotation.map(f32::to_degrees);
        let _ = writeln!(
            s,
            "$definebone {} {} {} {} {} {} 0 0 0 0 0 0",
            quote(&bone.name),
            quote(&parent_name(ir, bone)),
            vec3(bone.position),
            num(ry),
            num(rz),
            num(rx),
        );
    }
    for bone in &ir.bones {
        if bone.flags & BONE_USED_BY_BONE_MERGE != 0 {
            let _ = writeln!(s, "$bonemerge {}", quote(&bone.name));
        }
        // Model-space rotation bit: only VTMB fork compilers accept the
        // command, so it stays a comment that stock studiomdl ignores.
        if ir.version == 2531 && bone.flags & 0x02 != 0 {
            let _ = writeln!(s, "// $vtmbbonerotation {} \"model\"", quote(&bone.name));
        }
        let prop = &bone.surface_prop;
        if !prop.is_empty() && *prop != ir.surface_prop {
            let _ = writeln!(s, "$jointsurfaceprop {} {}", quote(&bone.name), quote(prop));
        }
    }
}

fn parent_name(ir: &ModelIr, bone: &Bone) -> String {
    let p = usize::try_from(bone.parent)
        .ok()
        .and_then(|p| ir.bones.get(p));
    p.map_or_else(String::new, |p| p.name.clone())
}
