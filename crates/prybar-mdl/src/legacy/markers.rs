use super::header::LegacyHeader;
use super::layout::Layout;
use crate::ir::{Attachment, Hitbox, HitboxSet};
use crate::read::{mat3x4, name_at, table, vec3, Source};
use crate::Result;
use prybar_io::Cursor;

pub fn attachments(r: &Source, hdr: &LegacyHeader, lay: &Layout) -> Result<Vec<Attachment>> {
    let Some(slot) = lay.attachments else {
        return Ok(Vec::new());
    };
    let (count, index) = hdr.span(slot);
    table(r, 0, count, index, 60)?
        .into_iter()
        .map(|at| {
            let mut c = Cursor::new(r.reader, at);
            let name_rel: i32 = c.read()?;
            Ok(Attachment {
                flags: c.read()?,
                bone: c.read()?,
                local: mat3x4(&mut c)?,
                name: name_at(r, at, name_rel)?,
            })
        })
        .collect()
}

pub fn hitbox_sets(r: &Source, hdr: &LegacyHeader, lay: &Layout) -> Result<Vec<HitboxSet>> {
    let (count, index) = hdr.span(lay.hitbox_sets);
    table(r, 0, count, index, 12)?
        .into_iter()
        .map(|at| {
            let mut c = Cursor::new(r.reader, at);
            let name_rel: i32 = c.read()?;
            let (n, rel) = (c.read()?, c.read()?);
            let stride = if lay.hitbox_named { 68 } else { 32 };
            let hitboxes = table(r, at, n, rel, stride)?
                .into_iter()
                .map(|at| hitbox(r, at, lay.hitbox_named))
                .collect::<Result<_>>()?;
            Ok(HitboxSet {
                name: name_at(r, at, name_rel)?,
                hitboxes,
            })
        })
        .collect()
}

/// Hitbox names are cosmetic; a bad offset must not fail the model.
fn hitbox(r: &Source, at: usize, named: bool) -> Result<Hitbox> {
    let mut c = Cursor::new(r.reader, at);
    let mut h = Hitbox {
        bone: c.read()?,
        group: c.read()?,
        min: vec3(&mut c)?,
        max: vec3(&mut c)?,
        ..Hitbox::default()
    };
    if named {
        let rel: i32 = c.read()?;
        h.name = name_at(r, at, rel).unwrap_or_default();
    }
    Ok(h)
}
