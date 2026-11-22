//! Procedural bone (VRD) file for `$proceduralbones`: quaternion helpers
//! and aim constraints.

mod rules;

use prybar_mdl::ir::ProcBone;
use prybar_mdl::ModelIr;

/// `None` when the model has no quaternion or aim bones; jiggle bones
/// belong in the QC instead.
pub fn vrd(ir: &ModelIr) -> Option<String> {
    let mut s = String::new();
    for (i, bone) in ir.bones.iter().enumerate() {
        match &bone.procedural {
            Some(ProcBone::QuatInterp { control, triggers }) => {
                rules::helper(&mut s, ir, i, *control, triggers)
            }
            Some(ProcBone::AimAtBone(aim)) => {
                rules::aim(&mut s, ir, i, aim, &bone_name(ir, aim.aim))
            }
            Some(ProcBone::AimAtAttachment(aim)) => {
                let target = usize::try_from(aim.aim)
                    .ok()
                    .and_then(|a| ir.attachments.get(a))
                    .map_or("", |a| a.name.as_str());
                rules::aim(&mut s, ir, i, aim, &short(target))
            }
            _ => {}
        }
    }
    (!s.is_empty()).then_some(s)
}

/// VRD names drop the `ValveBiped.` prefix, as Valve's own files do, and
/// studiomdl splits VRD lines on whitespace and treats `_` as a space.
fn short(name: &str) -> String {
    name.strip_prefix("ValveBiped.")
        .unwrap_or(name)
        .replace(char::is_whitespace, "_")
}

/// Out-of-range indices give an empty name rather than a panic.
fn bone_name(ir: &ModelIr, index: i32) -> String {
    let name = usize::try_from(index)
        .ok()
        .and_then(|i| ir.bones.get(i))
        .map_or("", |b| b.name.as_str());
    short(name)
}
