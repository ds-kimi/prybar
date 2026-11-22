use crate::flex::{eyeballs, mesh_flexes, DeltaCodec};
use crate::ir::{Mesh, SubModel};
use crate::read::Source;
use crate::read::{table, vec3};
use crate::Result;
use prybar_io::Cursor;

const MESH_STRIDE: usize = 116;

pub fn model(r: &Source, at: usize, codec: DeltaCodec) -> Result<SubModel> {
    let mut c = Cursor::new(r.reader, at);
    let raw = c.bytes(64)?;
    let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
    let kind = c.read()?;
    let bounding_radius = c.read()?;
    let mesh_count: i32 = c.read()?;
    let mesh_index: i32 = c.read()?;
    let mut m = SubModel {
        name: String::from_utf8_lossy(&raw[..end]).into_owned(),
        kind,
        bounding_radius,
        vertex_count: c.read()?,
        vertex_index: c.read()?,
        tangents_index: c.read()?,
        ..SubModel::default()
    };
    c.bytes(8)?;
    m.eyeball_count = c.read()?;
    m.eyeballs = eyeballs(r, at, m.eyeball_count, c.read()?)?;
    m.meshes = table(r, at, mesh_count, mesh_index, MESH_STRIDE)?
        .into_iter()
        .map(|at| mesh(r, at, codec))
        .collect::<Result<_>>()?;
    Ok(m)
}

fn mesh(r: &Source, at: usize, codec: DeltaCodec) -> Result<Mesh> {
    let mut c = Cursor::new(r.reader, at);
    let material = c.read()?;
    c.bytes(4)?;
    let vertex_count = c.read()?;
    let vertex_offset = c.read()?;
    let flex_count = c.read()?;
    let flex_index = c.read()?;
    // materialtype and materialparam carry nothing the IR needs.
    c.bytes(8)?;
    let mut m = Mesh {
        material,
        vertex_count,
        vertex_offset,
        flex_count,
        id: c.read()?,
        center: vec3(&mut c)?,
        flexes: mesh_flexes(r, at, flex_count, flex_index, codec)?,
        ..Mesh::default()
    };
    // Runtime vertex-data pointer, meaningless on disk.
    c.bytes(4)?;
    for n in &mut m.lod_vertex_counts {
        *n = c.read()?;
    }
    Ok(m)
}
