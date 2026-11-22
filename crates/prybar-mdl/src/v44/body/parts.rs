use super::model::model;
use crate::flex::DeltaCodec;
use crate::header::{fields, fields::BODY_PARTS, Header};
use crate::ir::BodyPart;
use crate::read::Source;
use crate::read::{name_at, table};
use crate::Result;
use prybar_io::Cursor;

const STRIDE: usize = 16;
/// `mstudiomodel_t` size for v44-v49.
const MODEL_STRIDE: usize = 148;

pub fn body_parts(r: &Source, hdr: &Header) -> Result<Vec<BodyPart>> {
    let (count, index) = hdr.span(BODY_PARTS);
    let codec = codec(hdr);
    table(r, 0, count, index, STRIDE)?
        .into_iter()
        .map(|at| body_part(r, at, codec))
        .collect()
}

fn codec(hdr: &Header) -> DeltaCodec {
    if hdr.int(fields::FLAGS) & fields::VERT_ANIM_FIXED_POINT != 0 {
        DeltaCodec::Fixed(hdr.float(fields::VERT_ANIM_SCALE))
    } else {
        DeltaCodec::Half
    }
}

fn body_part(r: &Source, at: usize, codec: DeltaCodec) -> Result<BodyPart> {
    let mut c = Cursor::new(r.reader, at);
    let name_rel: i32 = c.read()?;
    let count: i32 = c.read()?;
    let base = c.read()?;
    let index: i32 = c.read()?;
    let models = table(r, at, count, index, MODEL_STRIDE)?
        .into_iter()
        .map(|at| model(r, at, codec))
        .collect::<Result<_>>()?;
    Ok(BodyPart {
        name: name_at(r, at, name_rel)?,
        base,
        models,
    })
}
