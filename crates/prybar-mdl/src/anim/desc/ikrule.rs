use crate::anim::decode::AnimData;
use crate::ir::IkRule;
use crate::read::{name_at, table, Source, Words};
use crate::Result;

/// `mstudioikrule_t` size.
const STRIDE: usize = 152;

/// IK rules sit next to the animdesc, or, for anims stored in an `.ani`
/// block, in that block (`animblockikruleindex`). Missing `.ani` gives
/// no rules rather than an error, like missing frames.
pub fn ik_rules(r: &Source, d: &AnimData, desc: &Words, at: usize) -> Result<Vec<IkRule>> {
    let (count, block, ani_index) = (desc.i(15), desc.i(13), desc.i(17));
    if desc.i(16) != 0 {
        return read_rules(r, at, count, desc.i(16));
    }
    if count == 0 || block == 0 || ani_index == 0 {
        return Ok(Vec::new());
    }
    match d.locate(block, ani_index, at)? {
        Some((src, base)) => read_rules(src, base, count, 0),
        None => Ok(Vec::new()),
    }
}

fn read_rules(r: &Source, base: usize, count: i32, rel: i32) -> Result<Vec<IkRule>> {
    table(r, base, count, rel, STRIDE)?
        .into_iter()
        .map(|at| rule(r, at))
        .collect()
}

fn rule(r: &Source, at: usize) -> Result<IkRule> {
    let w = Words::read(r, at, STRIDE / 4)?;
    let q = w.v3(11);
    Ok(IkRule {
        index: w.i(0),
        kind: w.i(1),
        chain: w.i(2),
        bone: w.i(3),
        slot: w.i(4),
        height: w.f(5),
        radius: w.f(6),
        floor: w.f(7),
        position: w.v3(8),
        quat: [q[0], q[1], q[2], w.f(14)],
        influence: [w.f(19), w.f(20), w.f(21), w.f(22)],
        contact: w.f(24),
        drop: w.f(25),
        top: w.f(26),
        attachment: name_at(r, at, w.i(30))?,
    })
}
