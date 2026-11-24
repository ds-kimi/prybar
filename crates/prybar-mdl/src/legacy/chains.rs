use super::header::LegacyHeader;
use super::layout::Layout;
use crate::ir::PhysicsChain;
use crate::read::{table, Source, Words};
use crate::Result;

/// 28-byte records: bone, parent, then five floats. Out-of-range bone
/// indices are kept as read; the QC writer skips chains it cannot name.
pub fn physics_chains(r: &Source, hdr: &LegacyHeader, lay: &Layout) -> Result<Vec<PhysicsChain>> {
    let Some(slot) = lay.physics_chains else {
        return Ok(Vec::new());
    };
    let (count, offset) = hdr.span(slot);
    table(r, 0, count, offset, 28)?
        .into_iter()
        .map(|at| {
            let w = Words::read(r, at, 7)?;
            Ok(PhysicsChain {
                bone: w.i(0),
                parent: w.i(1),
                params: [2, 3, 4, 5, 6].map(|n| w.f(n)),
            })
        })
        .collect()
}
