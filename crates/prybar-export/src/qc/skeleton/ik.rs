use crate::qc::fmt::{num, quote};
use prybar_mdl::ir::IkLock;
use prybar_mdl::ModelIr;
use std::fmt::Write;

/// Chains go before `$animation` and `$sequence`: their `iklock` and
/// `ikrule` options refer to chains by name.
pub fn write(s: &mut String, ir: &ModelIr) {
    for chain in &ir.ik_chains {
        let bone = usize::try_from(chain.bone)
            .ok()
            .and_then(|b| ir.bones.get(b));
        let Some(bone) = bone else {
            continue;
        };
        let [x, y, z] = chain.knee.map(num);
        let _ = writeln!(
            s,
            "$ikchain {} {} knee {x} {y} {z}",
            quote(&chain.name),
            quote(&bone.name)
        );
    }
    for lock in &ir.ik_auto_locks {
        if let Some(line) = lock_args(ir, lock) {
            let _ = writeln!(s, "$ikautoplaylock {line}");
        }
    }
}

/// `<chain> <posweight> <localqweight>`, shared by `$ikautoplaylock` and
/// the sequence `iklock` option; `None` for a chain index out of range.
pub fn lock_args(ir: &ModelIr, lock: &IkLock) -> Option<String> {
    let chain = ir.ik_chains.get(usize::try_from(lock.chain).ok()?)?;
    Some(format!(
        "{} {} {}",
        quote(&chain.name),
        num(lock.pos_weight),
        num(lock.local_q_weight)
    ))
}
