use crate::qc::flex;
use crate::qc::fmt::quote;
use crate::qc::Names;
use prybar_mdl::ir::{BodyPart, SubModel};
use prybar_mdl::ModelIr;
use std::fmt::Write;

/// A lone sub-model is a `$body`, or a `$model` when it carries flexes or
/// eyeballs; several make a `$bodygroup`, with `blank` for empty entries.
pub fn write(s: &mut String, ir: &ModelIr, names: &Names) {
    for (pi, part) in ir.body_parts.iter().enumerate() {
        match (part.models.as_slice(), names.reference(pi, 0, 0)) {
            ([model], Some(file)) if is_flexed(model) => {
                flex::model_block(s, ir, names, (pi, model), file);
            }
            ([_], Some(file)) => {
                let _ = writeln!(s, "$body {} {}", quote(&part.name), quote(file));
            }
            _ => bodygroup(s, part, names, pi),
        }
    }
}

pub fn is_flexed(model: &SubModel) -> bool {
    !model.eyeballs.is_empty() || model.meshes.iter().any(|m| !m.flexes.is_empty())
}

fn bodygroup(s: &mut String, part: &BodyPart, names: &Names, pi: usize) {
    let _ = writeln!(s, "$bodygroup {} {{", quote(&part.name));
    for mi in 0..part.models.len() {
        match names.reference(pi, mi, 0) {
            Some(file) => {
                let _ = writeln!(s, "  studio {}", quote(file));
            }
            None => s.push_str("  blank\n"),
        }
    }
    s.push_str("}\n");
}
