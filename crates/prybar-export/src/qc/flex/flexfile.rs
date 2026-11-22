use super::eyes::is_lid;
use crate::qc::fmt::quote;
use crate::qc::sanitize::ident;
use crate::vta::frames::frames;
use prybar_mdl::ir::SubModel;
use prybar_mdl::ModelIr;
use std::fmt::Write;

/// Maps each VTA frame back to a flex desc; frame 0 is the base shape.
pub fn write(s: &mut String, ir: &ModelIr, model: &SubModel, vta: &str) {
    let frames = frames(ir, model);
    if frames.is_empty() {
        return;
    }
    let _ = writeln!(s, "  flexfile {} {{\n    defaultflex frame 0", quote(vta));
    for (i, f) in frames.iter().enumerate() {
        let n = i + 1;
        if f.parts.first().is_some_and(|(_, k)| is_lid(model, k.desc)) {
            continue;
        }
        match pair_name(&f.label).map(ident) {
            Some(base) => {
                let _ = writeln!(s, "    flexpair {} 1 frame {n}", quote(&base));
            }
            None => {
                let _ = writeln!(s, "    flex {} frame {n}", quote(&ident(&f.label)));
            }
        }
    }
    s.push_str("  }\n");
}

/// studiomdl names the halves of a `flexpair` `<name>R` (the desc) and
/// `<name>L` (its partner); either order of a stereo label maps back.
fn pair_name(label: &str) -> Option<&str> {
    let (first, second) = label.split_once('+')?;
    let halves = |a: char, b: char| {
        let base = first.strip_suffix(a)?;
        (second.strip_suffix(b)? == base && !base.is_empty()).then_some(base)
    };
    halves('R', 'L').or_else(|| halves('L', 'R'))
}
