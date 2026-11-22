use crate::header::{fields::BONES, Header};
use crate::ir::Bone;
use crate::read::Source;
use crate::read::{mat3x4, name_at, quat, table, vec3};
use crate::Result;
use prybar_io::Cursor;

/// `mstudiobone_t` size for v44-v49.
const STRIDE: usize = 216;

pub fn bones(r: &Source, hdr: &Header) -> Result<Vec<Bone>> {
    let (count, index) = hdr.span(BONES);
    table(r, 0, count, index, STRIDE)?
        .into_iter()
        .map(|at| bone(r, at, hdr.version))
        .collect()
}

fn bone(r: &Source, at: usize, version: i32) -> Result<Bone> {
    let mut c = Cursor::new(r.reader, at);
    let name_rel: i32 = c.read()?;
    let parent = c.read()?;
    // Bone controller slots are rebuilt from the controller table instead.
    c.bytes(24)?;
    let mut b = Bone {
        name: name_at(r, at, name_rel)?,
        parent,
        position: vec3(&mut c)?,
        quat: quat(&mut c)?,
        rotation: vec3(&mut c)?,
        position_scale: vec3(&mut c)?,
        rotation_scale: vec3(&mut c)?,
        pose_to_bone: mat3x4(&mut c)?,
        ..Bone::default()
    };
    // qAlignment is recomputed by studiomdl.
    c.bytes(16)?;
    b.flags = c.read()?;
    let kind = c.read()?;
    let index = c.read()?;
    // A stale procindex must not reject a model that otherwise loads.
    b.procedural = crate::procbones::parse(r, at, (kind, index), version).unwrap_or_default();
    b.physics_bone = c.read()?;
    let surface_rel: i32 = c.read()?;
    b.surface_prop = name_at(r, at, surface_rel)?;
    b.contents = c.read()?;
    Ok(b)
}
