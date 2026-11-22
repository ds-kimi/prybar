use crate::header::{fields::BONES, Header};
use crate::ir::Bone;
use crate::read::{mat3x4, name_at, quat, table, vec3, Source};
use crate::Result;
use prybar_io::Cursor;

/// `mstudiobone_t` size for v53.
const STRIDE: usize = 244;

pub fn bones(r: &Source, hdr: &Header) -> Result<Vec<Bone>> {
    let (count, index) = hdr.span(BONES);
    table(r, 0, count, index, STRIDE)?
        .into_iter()
        .map(|at| bone(r, at))
        .collect()
}

/// Same fields as v49 with two unknown vectors slotted in after the
/// rotation and after the rotation scale.
fn bone(r: &Source, at: usize) -> Result<Bone> {
    let mut c = Cursor::new(r.reader, at);
    let name_rel: i32 = c.read()?;
    let parent = c.read()?;
    c.bytes(24)?;
    let position = vec3(&mut c)?;
    let quat = quat(&mut c)?;
    let rotation = vec3(&mut c)?;
    c.bytes(12)?;
    let position_scale = vec3(&mut c)?;
    let rotation_scale = vec3(&mut c)?;
    c.bytes(12)?;
    let pose_to_bone = mat3x4(&mut c)?;
    c.bytes(16)?;
    let mut b = Bone {
        name: name_at(r, at, name_rel)?,
        parent,
        position,
        quat,
        rotation,
        position_scale,
        rotation_scale,
        pose_to_bone,
        ..Bone::default()
    };
    tail(r, at, &mut c, &mut b)?;
    Ok(b)
}

fn tail(r: &Source, at: usize, c: &mut Cursor, b: &mut Bone) -> Result<()> {
    b.flags = c.read()?;
    let rule = (c.read()?, c.read()?);
    // A stale procindex must not reject a model that otherwise loads.
    b.procedural = crate::procbones::parse(r, at, rule, 53).unwrap_or_default();
    b.physics_bone = c.read()?;
    let surface_rel: i32 = c.read()?;
    b.surface_prop = name_at(r, at, surface_rel)?;
    b.contents = c.read()?;
    Ok(())
}
