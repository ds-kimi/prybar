use crate::header::{fields::HITBOX_SETS, Header};
use crate::ir::{Hitbox, HitboxSet};
use crate::read::Source;
use crate::read::{name_at, table, vec3};
use crate::Result;
use prybar_io::Cursor;

const SET_STRIDE: usize = 12;
const BOX_STRIDE: usize = 68;

pub fn hitbox_sets(r: &Source, hdr: &Header) -> Result<Vec<HitboxSet>> {
    let (count, index) = hdr.span(HITBOX_SETS);
    table(r, 0, count, index, SET_STRIDE)?
        .into_iter()
        .map(|at| hitbox_set(r, at))
        .collect()
}

fn hitbox_set(r: &Source, at: usize) -> Result<HitboxSet> {
    let mut c = Cursor::new(r.reader, at);
    let name_rel: i32 = c.read()?;
    let count: i32 = c.read()?;
    let index: i32 = c.read()?;
    let hitboxes = table(r, at, count, index, BOX_STRIDE)?
        .into_iter()
        .map(|at| hitbox(r, at))
        .collect::<Result<_>>()?;
    Ok(HitboxSet {
        name: name_at(r, at, name_rel)?,
        hitboxes,
    })
}

fn hitbox(r: &Source, at: usize) -> Result<Hitbox> {
    let mut c = Cursor::new(r.reader, at);
    let bone = c.read()?;
    let group = c.read()?;
    let min = vec3(&mut c)?;
    let max = vec3(&mut c)?;
    let name_rel: i32 = c.read()?;
    Ok(Hitbox {
        name: hitbox_name(r, at, name_rel),
        bone,
        group,
        min,
        max,
    })
}

/// studiomdl documents this offset as relative, but v44-v48 files in the
/// wild store it as absolute, so both are tried. The name is cosmetic and
/// must never make an otherwise valid model unreadable.
fn hitbox_name(r: &Source, at: usize, rel: i32) -> String {
    name_at(r, at, rel)
        .or_else(|_| name_at(r, 0, rel))
        .unwrap_or_default()
}
