use super::lookup;
use prybar_mdl::ir::FlexOp;
use prybar_mdl::ModelIr;
use std::collections::HashSet;
use std::fmt::Write;

/// Rule targets and `%flex` operands with no shape behind them are
/// studiomdl local variables, and it rejects rules for undeclared ones.
/// One bare name per line: the form studiomdl accepted on HL2 faces.
pub fn write(s: &mut String, ir: &ModelIr) {
    let shaped = shaped(ir);
    let mut vars: Vec<i32> = Vec::new();
    for rule in &ir.flex_rules {
        let reads = rule.ops.iter().filter_map(|op| match op {
            FlexOp::FetchFlex(d) => Some(*d),
            _ => None,
        });
        for d in std::iter::once(rule.flex).chain(reads) {
            if !shaped.contains(&d) && !vars.contains(&d) {
                vars.push(d);
            }
        }
    }
    for d in vars {
        let _ = writeln!(s, "  localvar {}", lookup::desc(ir, d));
    }
}

/// Descs backed by a VTA frame or made by an `eyelid` line.
fn shaped(ir: &ModelIr) -> HashSet<i32> {
    let models = ir.body_parts.iter().flat_map(|p| &p.models);
    let mut out = HashSet::new();
    for m in models {
        for f in m.meshes.iter().flat_map(|mesh| &mesh.flexes) {
            out.extend([f.desc, f.partner]);
        }
        // Eyelid lines need the VTA, which only exists with flex frames.
        let has_vta = m.meshes.iter().any(|mesh| !mesh.flexes.is_empty());
        for e in m.eyeballs.iter().filter(|_| has_vta) {
            out.extend(e.upper_flex_descs.iter().chain(&e.lower_flex_descs));
            out.extend([e.upper_lid_flex_desc, e.lower_lid_flex_desc]);
        }
    }
    out
}
