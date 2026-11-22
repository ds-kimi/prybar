use crate::header::{fields, Header};
use crate::ir::Material;
use crate::read::Source;
use crate::read::{name_at, offset, table};
use crate::{Error, Result};

const TEXTURE_STRIDE: usize = 64;

pub fn materials(r: &Source, hdr: &Header) -> Result<Vec<Material>> {
    let (count, index) = hdr.span(fields::TEXTURES);
    table(r, 0, count, index, TEXTURE_STRIDE)?
        .into_iter()
        .map(|at| {
            Ok(Material {
                name: name_at(r, at, r.read(at)?)?,
                flags: r.read(at + 4)?,
            })
        })
        .collect()
}

/// `$cdmaterials` entries: an array of file-absolute string offsets.
pub fn material_dirs(r: &Source, hdr: &Header) -> Result<Vec<String>> {
    let (count, index) = hdr.span(fields::CD_TEXTURES);
    table(r, 0, count, index, 4)?
        .into_iter()
        .map(|at| name_at(r, 0, r.read(at)?))
        .collect()
}

pub fn skins(r: &Source, hdr: &Header) -> Result<Vec<Vec<u16>>> {
    let refs = hdr.int(fields::SKIN_REF_COUNT);
    let families = hdr.int(fields::SKIN_FAMILY_COUNT);
    let total = i64::from(refs) * i64::from(families);
    let cells = i32::try_from(total).map_err(|_| Error::Invalid {
        what: "skin table size",
        value: total,
    })?;
    let rows = table(r, 0, cells, hdr.int(fields::SKIN_INDEX), 2)?;
    let start = offset(0, hdr.int(fields::SKIN_INDEX))?;
    let flat: Vec<u16> = r.array(start, rows.len(), rows.len())?;
    let width = usize::try_from(refs).unwrap_or(0).max(1);
    Ok(flat.chunks(width).map(<[u16]>::to_vec).collect())
}
