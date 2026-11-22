use crate::qc::fmt::quote;
use prybar_mdl::ModelIr;
use std::fmt::Write;

/// `$texturegroup` lists only the slots that differ between families;
/// the others come from the SMD materials, as studiomdl expects.
pub fn write(s: &mut String, ir: &ModelIr) {
    let Some(first) = ir.skins.first().filter(|_| ir.skins.len() > 1) else {
        return;
    };
    let slots: Vec<usize> = (0..first.len())
        .filter(|&i| ir.skins.iter().any(|f| f.get(i) != first.get(i)))
        .collect();
    if slots.is_empty() {
        return;
    }
    s.push_str("$texturegroup \"skinfamilies\"\n{\n");
    for family in &ir.skins {
        s.push_str("  {");
        for &slot in &slots {
            let _ = write!(s, " {}", quote(&material(ir, family.get(slot))));
        }
        s.push_str(" }\n");
    }
    s.push_str("}\n");
}

fn material(ir: &ModelIr, index: Option<&u16>) -> String {
    let m = index.and_then(|&i| ir.materials.get(usize::from(i)));
    m.map_or_else(String::new, |m| m.name.clone())
}
