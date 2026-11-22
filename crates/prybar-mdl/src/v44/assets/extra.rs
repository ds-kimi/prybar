use crate::header::{fields, Header};
use crate::ir::IncludeModel;
use crate::read::Source;
use crate::read::{name_at, offset, table};
use crate::Result;

pub fn include_models(r: &Source, hdr: &Header) -> Result<Vec<IncludeModel>> {
    let (count, index) = hdr.span(fields::INCLUDE_MODELS);
    table(r, 0, count, index, 8)?
        .into_iter()
        .map(|at| {
            Ok(IncludeModel {
                label: name_at(r, at, r.read(at)?)?,
                path: name_at(r, at, r.read(at + 4)?)?,
            })
        })
        .collect()
}

pub fn key_values(r: &Source, hdr: &Header) -> Result<String> {
    let size = hdr.int(fields::KEY_VALUE_SIZE);
    let index = hdr.int(fields::KEY_VALUE_INDEX);
    if size <= 0 || index <= 0 {
        return Ok(String::new());
    }
    let raw = r.bytes(offset(0, index)?, size as usize)?;
    let text = String::from_utf8_lossy(raw);
    Ok(text.trim_end_matches('\0').to_owned())
}

pub fn surface_prop(r: &Source, hdr: &Header) -> Result<String> {
    name_at(r, 0, hdr.int(fields::SURFACE_PROP))
}
