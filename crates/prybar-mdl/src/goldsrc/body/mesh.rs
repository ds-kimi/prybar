use super::tris::Corner;
use super::verts::{vertex, Placed};
use super::{tris, Ctx};
use crate::ir::{LodMesh, Mesh, SubModel, Vertex};
use crate::read::Words;
use crate::Result;
use std::collections::HashMap;

/// Model points and normals, both in model space.
type Points<'a> = (&'a [Placed], &'a [Placed]);

/// `mstudiomesh_t`: tris, tri index, skin ref, norms, norm index. One
/// vertex per distinct corner: strips repeat corners, and sharing them
/// keeps a forged strip from costing far more memory than its bytes.
pub fn mesh(ctx: &Ctx, at: usize, pts: (&[Placed], &[Placed]), sub: &mut SubModel) -> Result<()> {
    let w = Words::read(ctx.r, at, 5)?;
    let slot = w.i(2);
    let dims = usize::try_from(slot)
        .ok()
        .and_then(|s| ctx.dims.get(s).copied())
        .unwrap_or((1.0, 1.0));
    let mut geo = LodMesh::default();
    let mut seen = HashMap::new();
    for tri in tris::triangles(ctx.r, w.i(1))? {
        let mut ids = [0u32; 3];
        for (id, corner) in ids.iter_mut().zip(tri) {
            *id = share(&mut seen, &mut sub.vertices, corner, (pts, dims))?;
        }
        geo.triangles.push(ids);
    }
    sub.meshes.push(Mesh {
        material: slot,
        vertex_count: geo.triangles.len() as i32 * 3,
        ..Mesh::default()
    });
    sub.lods[0].meshes.push(geo);
    Ok(())
}

fn share(
    seen: &mut HashMap<Corner, u32>,
    verts: &mut Vec<Vertex>,
    corner: Corner,
    (pts, dims): (Points, (f32, f32)),
) -> Result<u32> {
    if let Some(&i) = seen.get(&corner) {
        return Ok(i);
    }
    let i = verts.len() as u32;
    verts.push(vertex(pts, corner, dims)?);
    seen.insert(corner, i);
    Ok(i)
}
