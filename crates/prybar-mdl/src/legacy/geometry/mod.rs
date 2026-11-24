//! MDL 2531 keeps each model's vertices in the MDL, in one of three
//! formats chosen per model; triangles still come from the VTX (v107).

pub mod normals;
mod vertex;

use super::face;
use super::header::LegacyHeader;
use super::layout::Layout;
use crate::ir::{Mesh, ModelIr, SubModel, Vertex};
use crate::read::{table, Source, Words};
use crate::Result;
use vertex::Frame;

/// `StudioMesh` with its vertex data block: 15 ints.
const MESH_STRIDE: usize = 60;

/// Fills meshes and vertices of every model. A model that does not decode
/// is left empty and named in the notes, so the skeleton still exports.
pub fn attach(r: &Source, hdr: &LegacyHeader, lay: &Layout, ir: &mut ModelIr) -> Result<()> {
    let (count, index) = hdr.span(lay.body_parts);
    let parts = table(r, 0, count, index, 16)?;
    for (part, at) in ir.body_parts.iter_mut().zip(parts) {
        let (n, rel): (i32, i32) = (r.read(at + 4)?, r.read(at + 12)?);
        let models = table(r, at, n, rel, lay.model_stride)?;
        for (model, at) in part.models.iter_mut().zip(models) {
            if let Err(e) = load(r, at + lay.model_name_len, at, model) {
                ir.notes.push(format!("meshes of {}: {e}", model.name));
                model.meshes.clear();
                model.vertices.clear();
            }
        }
    }
    Ok(())
}

/// `fields` is just past the model name; offsets are model-relative.
fn load(r: &Source, fields: usize, at: usize, model: &mut SubModel) -> Result<()> {
    let w = Words::read(r, fields, 18)?;
    let mut kinds = Vec::new();
    model.meshes = table(r, at, w.i(2), w.i(3), MESH_STRIDE)?
        .into_iter()
        .map(|m| {
            let words = Words::read(r, m, 8)?;
            kinds.push((words.i(6), words.i(7), words.i(0)));
            mesh(r, m, &words)
        })
        .collect::<Result<_>>()?;
    model.eyeballs = face::eyeballs(r, at, (w.i(16), w.i(17)), &kinds)?;
    model.eyeball_count = model.eyeballs.len() as i32;
    let frame = Frame {
        offset: w.v3(8),
        scale: w.v3(11),
    };
    model.vertices = vertices(r, at, (w.i(4), w.i(5)), w.i(7), &frame)?;
    Ok(())
}

/// Words 4 and 5 are the flex `(count, index)`, relative to the mesh.
fn mesh(r: &Source, at: usize, w: &Words) -> Result<Mesh> {
    let flexes = face::mesh_flexes(r, at, w.i(4), w.i(5))?;
    Ok(Mesh {
        material: w.i(0),
        vertex_count: w.i(2),
        vertex_offset: w.i(3),
        flex_count: flexes.len() as i32,
        flexes,
        ..Mesh::default()
    })
}

/// `kind` 0 is skinned, 1 unskinned, anything else compressed.
fn vertices(
    r: &Source,
    at: usize,
    span: (i32, i32),
    kind: i32,
    frame: &Frame,
) -> Result<Vec<Vertex>> {
    let stride = match kind {
        0 => 44,
        1 => 12,
        _ => 8,
    };
    table(r, at, span.0, span.1, stride)?
        .into_iter()
        .map(|v| match kind {
            0 => vertex::skinned(r, v),
            _ => vertex::packed(r, v, frame, kind == 1),
        })
        .collect()
}
