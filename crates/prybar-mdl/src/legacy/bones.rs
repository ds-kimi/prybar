use super::header::LegacyHeader;
use super::layout::{BoneShape, Layout};
use crate::ir::Bone;
use crate::read::{mat3x4, name_at, quat, table, vec3, Source};
use crate::Result;
use prybar_io::Cursor;

pub fn bones(r: &Source, hdr: &LegacyHeader, lay: &Layout) -> Result<Vec<Bone>> {
    let (count, index) = hdr.span(lay.bones);
    let stride = match lay.bone {
        BoneShape::V37 => 196,
        BoneShape::V31 => 148,
        BoneShape::V2531 => 136,
    };
    table(r, 0, count, index, stride)?
        .into_iter()
        .map(|at| match lay.bone {
            BoneShape::V2531 => bone_2531(r, at),
            shape => bone_euler(r, at, shape),
        })
        .collect()
}

/// v27-v37 keep the v44 field order up to the surface prop; v32+ then
/// append the quat and contents that v44 later moved forward.
fn bone_euler(r: &Source, at: usize, shape: BoneShape) -> Result<Bone> {
    let mut c = Cursor::new(r.reader, at);
    let name_rel: i32 = c.read()?;
    let mut b = Bone {
        parent: c.read()?,
        ..Bone::default()
    };
    // Bone controller slots are rebuilt from the controller table.
    c.bytes(24)?;
    b.position = vec3(&mut c)?;
    b.rotation = vec3(&mut c)?;
    b.position_scale = vec3(&mut c)?;
    b.rotation_scale = vec3(&mut c)?;
    b.pose_to_bone = mat3x4(&mut c)?;
    if shape == BoneShape::V37 {
        // qAlignment is recomputed by studiomdl.
        c.bytes(16)?;
    }
    tail(r, at, &mut c, &mut b)?;
    if shape == BoneShape::V37 {
        b.quat = quat(&mut c)?;
        b.contents = c.read()?;
    }
    b.name = name_at(r, at, name_rel)?;
    Ok(b)
}

/// 2531 stores a quat where later versions keep euler angles, and its
/// rotation scale is four floats whose last one has no known use.
fn bone_2531(r: &Source, at: usize) -> Result<Bone> {
    let mut c = Cursor::new(r.reader, at);
    let name_rel: i32 = c.read()?;
    let mut b = Bone {
        parent: c.read()?,
        position: vec3(&mut c)?,
        quat: quat(&mut c)?,
        position_scale: vec3(&mut c)?,
        rotation_scale: vec3(&mut c)?,
        ..Bone::default()
    };
    c.bytes(4)?;
    b.pose_to_bone = mat3x4(&mut c)?;
    tail(r, at, &mut c, &mut b)?;
    b.contents = c.read()?;
    b.name = name_at(r, at, name_rel)?;
    Ok(b)
}

/// Procedural rules are skipped: their legacy structs are not decoded.
fn tail(r: &Source, at: usize, c: &mut Cursor, b: &mut Bone) -> Result<()> {
    b.flags = c.read()?;
    c.bytes(8)?;
    b.physics_bone = c.read()?;
    let surface_rel: i32 = c.read()?;
    b.surface_prop = name_at(r, at, surface_rel)?;
    Ok(())
}
