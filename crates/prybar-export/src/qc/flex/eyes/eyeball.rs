use super::name::eye_name;
use crate::qc::fmt::{num, quote, vec3};
use crate::smd::phys_math::bone_to_model;
use crate::smd::triangles::material_name;
use prybar_mdl::ir::{Eyeball, SubModel};
use prybar_mdl::ModelIr;
use std::fmt::Write;

/// The MDL keeps the eyeball centre in bone space and the angle as
/// `tan(angle)` and the iris scale inverted; the QC wants model space,
/// degrees and the scale as typed. The iris material slot is unused since
/// the eyes shader, so a placeholder is written.
pub fn write(s: &mut String, ir: &ModelIr, model: &SubModel) {
    for (i, eye) in model.eyeballs.iter().enumerate() {
        let bone = usize::try_from(eye.bone).ok().and_then(|b| ir.bones.get(b));
        let origin = bone.map_or(eye.origin, |b| bone_to_model(&b.pose_to_bone, eye.origin));
        let _ = writeln!(
            s,
            "  eyeball {} {} {} {} {} {} \"iris_unused\" {}",
            quote(&eye_name(model, i)),
            quote(bone.map_or("", |b| b.name.as_str())),
            vec3(origin),
            quote(&material(ir, eye)),
            num(eye.radius * 2.0),
            num(eye.z_offset.atan().to_degrees()),
            num(iris(eye.iris_scale)),
        );
    }
}

/// Same slot-to-material mapping as the reference SMD, since studiomdl
/// matches the eyeball material by name against the SMD's materials.
fn material(ir: &ModelIr, eye: &Eyeball) -> String {
    material_name(ir, eye.texture)
}

fn iris(stored: f32) -> f32 {
    if stored.abs() > f32::EPSILON {
        1.0 / stored
    } else {
        0.0
    }
}
