use crate::qc::fmt::{num, quote};
use prybar_mdl::ModelIr;
use std::fmt::Write;

/// Distinct non-default bone weight lists as `$weightlist`s, and for each
/// sequence the list it uses. All-ones is studiomdl's default.
pub fn write(s: &mut String, ir: &ModelIr) -> Vec<Option<String>> {
    let mut lists: Vec<&[f32]> = Vec::new();
    let mut out = Vec::with_capacity(ir.sequences.len());
    for seq in &ir.sequences {
        let w = seq.bone_weights.as_slice();
        if w.iter().all(|&x| x == 1.0) {
            out.push(None);
            continue;
        }
        let i = lists.iter().position(|l| *l == w).unwrap_or_else(|| {
            lists.push(w);
            lists.len() - 1
        });
        out.push(Some(format!("weights_{i}")));
    }
    for (i, list) in lists.iter().enumerate() {
        let _ = writeln!(s, "$weightlist \"weights_{i}\" {{");
        for (w, bone) in list.iter().zip(&ir.bones) {
            let _ = writeln!(s, "  {} {}", quote(&bone.name), num(*w));
        }
        s.push_str("}\n");
    }
    out
}
