use prybar_mdl::ModelIr;
use std::collections::HashSet;

/// QC names of the animations. studiomdl keeps `$animation` and
/// `$sequence` names in one namespace, and inline anims are stored as
/// `@name`, so clashes get an `a_` prefix and then a number.
pub fn labels(ir: &ModelIr) -> Vec<String> {
    let mut taken: HashSet<String> = ir.sequences.iter().map(|s| s.name.to_lowercase()).collect();
    let mut out = Vec::with_capacity(ir.animations.len());
    for (i, a) in ir.animations.iter().enumerate() {
        let base: String = a
            .name
            .trim_start_matches('@')
            .chars()
            .filter(|c| *c != '"' && !c.is_control())
            .collect();
        let base = if base.is_empty() {
            format!("anim_{i}")
        } else {
            base
        };
        let mut name = base.clone();
        let mut n = 1;
        while !taken.insert(name.to_lowercase()) {
            name = match n {
                1 => format!("a_{base}"),
                _ => format!("a_{base}_{n}"),
            };
            n += 1;
        }
        out.push(name);
    }
    out
}
