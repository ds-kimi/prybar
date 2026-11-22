//! Facial animation data: flex descs, controllers, rules, eyeballs and the
//! per-mesh vertex animations that VTA files are rebuilt from.

mod eyeball;
mod rules;
mod vertanim;

pub use eyeball::eyeballs;
pub use vertanim::{mesh_flexes, DeltaCodec};

use crate::header::{fields, Header};
use crate::ir::FlexController;
use crate::read::Source;
use crate::read::{name_at, table};
use crate::{ModelIr, Result};

pub fn fill(r: &Source, hdr: &Header, ir: &mut ModelIr) -> Result<()> {
    let (count, index) = hdr.span(fields::FLEX_DESCS);
    ir.flex_descs = table(r, 0, count, index, 4)?
        .into_iter()
        .map(|at| name_at(r, at, r.read(at)?))
        .collect::<Result<_>>()?;
    ir.flex_controllers = controllers(r, hdr)?;
    ir.flex_rules = rules::rules(r, hdr)?;
    Ok(())
}

fn controllers(r: &Source, hdr: &Header) -> Result<Vec<FlexController>> {
    let (count, index) = hdr.span(fields::FLEX_CONTROLLERS);
    table(r, 0, count, index, 20)?
        .into_iter()
        .map(|at| {
            Ok(FlexController {
                kind: name_at(r, at, r.read(at)?)?,
                name: name_at(r, at, r.read(at + 4)?)?,
                min: r.read(at + 12)?,
                max: r.read(at + 16)?,
            })
        })
        .collect()
}
