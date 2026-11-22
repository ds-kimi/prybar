use crate::ir::{Lod, LodMesh, SubModel};
use crate::vtx::VtxLod;
use crate::{Error, Result};

pub fn build(model: &SubModel, vtx: &VtxLod) -> Result<Lod> {
    let meshes = model
        .meshes
        .iter()
        .enumerate()
        .map(|(k, mesh)| {
            let tris = vtx.meshes.get(k).map(Vec::as_slice).unwrap_or_default();
            mesh_triangles(tris, mesh.vertex_offset, model.vertices.len())
        })
        .collect::<Result<_>>()?;
    Ok(Lod {
        switch_point: vtx.switch_point,
        meshes,
    })
}

fn mesh_triangles(tris: &[[u16; 3]], offset: i32, len: usize) -> Result<LodMesh> {
    let base = u32::try_from(offset).map_err(|_| Error::Invalid {
        what: "mesh vertex offset",
        value: offset.into(),
    })?;
    let mut out = Vec::with_capacity(tris.len());
    for t in tris {
        let tri = t.map(|i| base.saturating_add(u32::from(i)));
        if let Some(&bad) = tri.iter().find(|&&i| i as usize >= len) {
            return Err(Error::Invalid {
                what: "triangle vertex index",
                value: bad.into(),
            });
        }
        out.push(tri);
    }
    Ok(LodMesh { triangles: out })
}
