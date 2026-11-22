use super::name::eye_name;
use crate::qc::flex::lookup;
use crate::qc::fmt::{num, quote};
use crate::vta::frames::{frames, Frame};
use prybar_mdl::ir::{Eyeball, SubModel};
use prybar_mdl::ModelIr;
use std::fmt::Write;

struct Lid<'a> {
    desc: i32,
    targets: [f32; 3],
    eye: &'a str,
    vta: &'a str,
}

/// studiomdl turns each `eyelid` into the `<lid>_lowerer/_neutral/_raiser`
/// descs the face rules use; plain `flex` lines would leave those rules
/// pointing at unknown flexes.
pub fn write(s: &mut String, ir: &ModelIr, model: &SubModel, vta: &str) {
    let frames = frames(ir, model);
    for (i, eye) in model.eyeballs.iter().enumerate() {
        if !has_lids(eye) {
            continue;
        }
        let name = eye_name(model, i);
        let lid = |desc, targets| Lid {
            desc,
            targets,
            eye: &name,
            vta,
        };
        line(
            s,
            ir,
            &frames,
            &lid(eye.upper_lid_flex_desc, eye.upper_targets),
        );
        line(
            s,
            ir,
            &frames,
            &lid(eye.lower_lid_flex_desc, eye.lower_targets),
        );
    }
}

/// A DMX `dummy_eyelid` reuses the lowerer desc as the lid desc; it has
/// no frames to bind.
fn has_lids(eye: &Eyeball) -> bool {
    eye.upper_lid_flex_desc != eye.upper_flex_descs[0] && eye.upper_lid_flex_desc >= 0
}

/// studiomdl tags the lowerer key with target -11 and the raiser with 11;
/// the neutral pose is the base frame. The split sign keeps the lid's side.
fn line(s: &mut String, ir: &ModelIr, frames: &[Frame], lid: &Lid) {
    let name = lookup::desc(ir, lid.desc);
    let at = |pick: fn([f32; 4]) -> bool| {
        frames
            .iter()
            .position(|f| {
                f.parts
                    .first()
                    .is_some_and(|(_, k)| k.desc == lid.desc && pick(k.targets))
            })
            .map_or(0, |i| i + 1)
    };
    let split = if name.contains("left") { "-0.1" } else { "0.1" };
    let t = lid.targets.map(num);
    let _ = writeln!(
        s,
        "  eyelid {name} {} lowerer {} {} neutral 0 {} raiser {} {} split {split} eyeball {}",
        quote(lid.vta),
        at(|t| t[0] <= -10.5),
        t[0],
        t[1],
        at(|t| t[3] >= 10.5),
        t[2],
        quote(lid.eye),
    );
}

/// Frames of a lid desc are written by the eyelid line, not as flexes.
pub fn is_lid(model: &SubModel, desc: i32) -> bool {
    model
        .eyeballs
        .iter()
        .filter(|e| has_lids(e))
        .any(|e| e.upper_lid_flex_desc == desc || e.lower_lid_flex_desc == desc)
}
