use crate::qc::fmt::{num, quote};
use crate::qc::Names;
use prybar_mdl::ModelIr;
use std::fmt::Write;

/// One `$lod` block per level past 0, replacing each sub-model's
/// reference SMD with its LOD SMD.
pub fn write(s: &mut String, ir: &ModelIr, names: &Names) {
    let models = || ir.body_parts.iter().flat_map(|p| &p.models);
    let levels = models().map(|m| m.lods.len()).max().unwrap_or(0);
    for lod in 1..levels {
        let switch = models()
            .find_map(|m| m.lods.get(lod))
            .map_or(0.0, |l| l.switch_point);
        let _ = writeln!(s, "{}{{", head(switch));
        for (pi, part) in ir.body_parts.iter().enumerate() {
            for mi in 0..part.models.len() {
                let pair = (names.reference(pi, mi, 0), names.reference(pi, mi, lod));
                if let (Some(base), Some(this)) = pair {
                    let _ = writeln!(s, "  replacemodel {} {}", quote(base), quote(this));
                }
            }
        }
        s.push_str("}\n");
    }
}

/// studiomdl stores the shadow LOD with a negative switch point. Brace
/// placement follows studiomdl's parser: `$lod` wants it on the command
/// line, `$shadowlod` (no argument) on the next line.
fn head(switch: f32) -> String {
    if switch < 0.0 {
        "$shadowlod\n".to_owned()
    } else {
        format!("$lod {} ", num(switch))
    }
}
