use crate::ir::AimAt;
use crate::read::{Source, Words};
use crate::Result;

/// `mstudioaimatbone_t`, shared by the bone and attachment variants.
pub fn aim_at(r: &Source, at: usize) -> Result<AimAt> {
    let w = Words::read(r, at, 11)?;
    Ok(AimAt {
        parent: w.i(0),
        aim: w.i(1),
        aim_vector: w.v3(2),
        up_vector: w.v3(5),
        base_pos: w.v3(8),
    })
}
