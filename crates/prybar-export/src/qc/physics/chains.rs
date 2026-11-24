use super::super::fmt::{num, quote};
use prybar_mdl::ModelIr;
use std::fmt::Write;

/// `$physicschain` lines for MDL 2531. Only VTMB fork compilers know the
/// command, so like `$vtmbbonerotation` it is written as a comment that
/// stock studiomdl ignores. A chain whose bone is out of range has no name
/// to write and is skipped; a missing parent is written as `""`.
pub fn write(ir: &ModelIr) -> String {
    let mut s = String::new();
    let name = |i: i32| {
        usize::try_from(i)
            .ok()
            .and_then(|i| ir.bones.get(i))
            .map(|b| b.name.as_str())
    };
    for chain in &ir.physics_chains {
        let Some(bone) = name(chain.bone) else {
            continue;
        };
        let parent = name(chain.parent).unwrap_or("");
        let params: Vec<String> = chain.params.iter().map(|&p| num(p)).collect();
        let _ = writeln!(
            s,
            "// $physicschain {} {} {}",
            quote(bone),
            quote(parent),
            params.join(" ")
        );
    }
    s
}
