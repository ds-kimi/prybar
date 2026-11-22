use super::header::LegacyHeader;
use super::layout::Layout;
use crate::ir::{BodyPart, Material, SubModel};
use crate::read::{name_at, offset, table, Source};
use crate::{Error, Result};
use prybar_io::Cursor;

pub fn materials(r: &Source, hdr: &LegacyHeader, lay: &Layout) -> Result<Vec<Material>> {
    let (count, index) = hdr.span(lay.textures);
    table(r, 0, count, index, lay.texture_stride)?
        .into_iter()
        .map(|at| {
            Ok(Material {
                name: name_at(r, at, r.read(at)?)?,
                flags: r.read(at + 4)?,
            })
        })
        .collect()
}

pub fn skins(r: &Source, hdr: &LegacyHeader, lay: &Layout) -> Result<Vec<Vec<u16>>> {
    let refs = hdr.int(lay.skins);
    let total = i64::from(refs) * i64::from(hdr.int(lay.skins + 1));
    let cells = i32::try_from(total).map_err(|_| Error::Invalid {
        what: "skin table size",
        value: total,
    })?;
    let index = hdr.int(lay.skins + 2);
    let n = table(r, 0, cells, index, 2)?.len();
    let flat: Vec<u16> = r.array(offset(0, index)?, n, n)?;
    let width = usize::try_from(refs).unwrap_or(0).max(1);
    Ok(flat.chunks(width).map(<[u16]>::to_vec).collect())
}

/// Meshes are left out: legacy models keep their vertices inside the MDL
/// in layouts that are not decoded, so only names and counts survive.
pub fn body_parts(r: &Source, hdr: &LegacyHeader, lay: &Layout) -> Result<Vec<BodyPart>> {
    let (count, index) = hdr.span(lay.body_parts);
    table(r, 0, count, index, 16)?
        .into_iter()
        .map(|at| {
            let mut c = Cursor::new(r.reader, at);
            let name_rel: i32 = c.read()?;
            let (n, base, rel): (i32, i32, i32) = (c.read()?, c.read()?, c.read()?);
            let models = table(r, at, n, rel, lay.model_stride)?
                .into_iter()
                .map(|at| model(r, at, lay.model_name_len))
                .collect::<Result<_>>()?;
            Ok(BodyPart {
                name: name_at(r, at, name_rel)?,
                base,
                models,
            })
        })
        .collect()
}

fn model(r: &Source, at: usize, name_len: usize) -> Result<SubModel> {
    let raw = r.bytes(at, name_len)?;
    let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
    let mut c = Cursor::new(r.reader, at + name_len);
    let kind = c.read()?;
    let bounding_radius = c.read()?;
    c.bytes(8)?;
    Ok(SubModel {
        name: String::from_utf8_lossy(&raw[..end]).into_owned(),
        kind,
        bounding_radius,
        vertex_count: c.read()?,
        ..SubModel::default()
    })
}
