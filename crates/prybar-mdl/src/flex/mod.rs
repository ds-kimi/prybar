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
    let spans = [
        hdr.span(fields::FLEX_DESCS),
        hdr.span(fields::FLEX_CONTROLLERS),
        hdr.span(fields::FLEX_RULES),
    ];
    tables(r, spans, ir)
}

/// Descs, controllers and rules from their `(count, index)` header pairs;
/// MDL 2531 stores the same three records, only at other header slots.
pub fn tables(r: &Source, spans: [(i32, i32); 3], ir: &mut ModelIr) -> Result<()> {
    let (count, index) = spans[0];
    ir.flex_descs = table(r, 0, count, index, 4)?
        .into_iter()
        .map(|at| name_at(r, at, r.read(at)?))
        .collect::<Result<_>>()?;
    ir.flex_controllers = controllers(r, spans[1])?;
    ir.flex_rules = rules::rules(r, spans[2])?;
    Ok(())
}

fn controllers(r: &Source, (count, index): (i32, i32)) -> Result<Vec<FlexController>> {
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
