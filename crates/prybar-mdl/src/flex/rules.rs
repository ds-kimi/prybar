use crate::ir::{FlexOp, FlexRule};
use crate::read::table;
use crate::read::Source;
use crate::Result;

pub fn rules(r: &Source, (count, index): (i32, i32)) -> Result<Vec<FlexRule>> {
    table(r, 0, count, index, 12)?
        .into_iter()
        .map(|at| {
            let ops = table(r, at, r.read(at + 4)?, r.read(at + 8)?, 8)?
                .into_iter()
                .map(|op| Ok(decode(r.read(op)?, r.read(op + 4)?)))
                .collect::<Result<_>>()?;
            Ok(FlexRule {
                flex: r.read(at)?,
                ops,
            })
        })
        .collect()
}

/// The payload is a union: a float for `STUDIO_CONST`, an index otherwise.
fn decode(op: i32, raw: i32) -> FlexOp {
    use FlexOp::*;
    match op {
        1 => Const(f32::from_bits(raw as u32)),
        2 => Fetch(raw),
        3 => FetchFlex(raw),
        4 => Add,
        5 => Sub,
        6 => Mul,
        7 => Div,
        8 => Neg,
        9 => Exp,
        10 => Open,
        11 => Close,
        12 => Comma,
        13 => Max,
        14 => Min,
        _ => indexed(op, raw),
    }
}

fn indexed(op: i32, raw: i32) -> FlexOp {
    use FlexOp::*;
    match op {
        15 => TwoWay0(raw),
        16 => TwoWay1(raw),
        17 => NWay(raw),
        18 => Combo(raw),
        19 => Dominate(raw),
        20 => DmeLowerEyelid(raw),
        21 => DmeUpperEyelid(raw),
        _ => Unknown { op, value: raw },
    }
}
