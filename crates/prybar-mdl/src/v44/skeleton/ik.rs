use crate::header::{fields, Header};
use crate::ir::{IkChain, IkLock};
use crate::read::{name_at, table, Source, Words};
use crate::Result;

/// `mstudioikchain_t`: name, link type, link count, link index.
const CHAIN: usize = 16;
/// `mstudioiklink_t`: bone, knee direction, unused vector.
const LINK: usize = 28;

pub fn ik_chains(r: &Source, hdr: &Header) -> Result<Vec<IkChain>> {
    let (count, index) = hdr.span(fields::IK_CHAINS);
    table(r, 0, count, index, CHAIN)?
        .into_iter()
        .map(|at| chain(r, at))
        .collect()
}

fn chain(r: &Source, at: usize) -> Result<IkChain> {
    let w = Words::read(r, at, 4)?;
    let links = table(r, at, w.i(2), w.i(3), LINK)?;
    let word = |i: usize, n: usize| links.get(i).map(|&l| Words::read(r, l, n)).transpose();
    let first = word(0, 4)?;
    let last = word(links.len().saturating_sub(1), 1)?;
    Ok(IkChain {
        name: name_at(r, at, w.i(0))?,
        bone: last.map_or(-1, |l| l.i(0)),
        knee: first.map_or([0.0; 3], |l| l.v3(1)),
    })
}

pub fn ik_auto_locks(r: &Source, hdr: &Header) -> Result<Vec<IkLock>> {
    let (count, index) = hdr.span(fields::IK_AUTOPLAY_LOCKS);
    crate::seq::ik_locks(r, 0, count, index)
}
