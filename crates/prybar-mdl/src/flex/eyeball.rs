use crate::ir::Eyeball;
use crate::read::Source;
use crate::read::{name_at, table, Words};
use crate::Result;

/// `mstudioeyeball_t` size.
const STRIDE: usize = 172;

/// Eyeballs of one `mstudiomodel_t`; the index is relative to the model.
pub fn eyeballs(r: &Source, model_at: usize, count: i32, index: i32) -> Result<Vec<Eyeball>> {
    table(r, model_at, count, index, STRIDE)?
        .into_iter()
        .map(|at| eyeball(r, at))
        .collect()
}

fn eyeball(r: &Source, at: usize) -> Result<Eyeball> {
    let w = Words::read(r, at, STRIDE / 4)?;
    let three = |n: usize| [w.i(n), w.i(n + 1), w.i(n + 2)];
    // The non-FACS flag is the low byte of the word after the 4 unused ints.
    let non_facs = w.i(35) & 0xff != 0;
    Ok(Eyeball {
        name: name_at(r, at, w.i(0))?,
        bone: w.i(1),
        origin: w.v3(2),
        z_offset: w.f(5),
        radius: w.f(6),
        up: w.v3(7),
        forward: w.v3(10),
        texture: w.i(13),
        iris_scale: w.f(15),
        upper_flex_descs: three(17),
        lower_flex_descs: three(20),
        upper_targets: w.v3(23),
        lower_targets: w.v3(26),
        upper_lid_flex_desc: w.i(29),
        lower_lid_flex_desc: w.i(30),
        non_facs,
    })
}
