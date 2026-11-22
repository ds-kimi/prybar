use prybar_mdl::ir::{SubModel, Vertex};
use prybar_mdl::ModelIr;
use std::fmt::Write;

pub fn write(s: &mut String, ir: &ModelIr, model: &SubModel, lod: usize) {
    s.push_str("triangles\n");
    let Some(lod) = model.lods.get(lod) else {
        s.push_str("end\n");
        return;
    };
    for (mesh, geo) in model.meshes.iter().zip(&lod.meshes) {
        let material = material_name(ir, mesh.material);
        for tri in &geo.triangles {
            let _ = writeln!(s, "{material}");
            for &i in tri {
                if let Some(v) = model.vertices.get(i as usize) {
                    vertex_line(s, v);
                }
            }
        }
    }
    s.push_str("end\n");
}

/// Meshes reference skin slots; the default skin family maps a slot to the
/// material that studiomdl must see in the reference SMD.
pub(crate) fn material_name(ir: &ModelIr, slot: i32) -> String {
    let slot = usize::try_from(slot).unwrap_or(0);
    let index = ir
        .skins
        .first()
        .and_then(|row| row.get(slot))
        .map_or(slot, |&m| usize::from(m));
    ir.materials
        .get(index)
        .map_or_else(|| format!("material{index}"), |m| m.name.clone())
}

/// SMD v grows upwards, Source v downwards.
fn vertex_line(s: &mut String, v: &Vertex) {
    let parent = v.weights.first().map_or(0, |w| w.0);
    let [px, py, pz] = v.position;
    let [nx, ny, nz] = v.normal;
    let (u, vv) = (v.uv[0], 1.0 - v.uv[1]);
    let _ = write!(
        s,
        "  {parent} {px:.6} {py:.6} {pz:.6} {nx:.6} {ny:.6} {nz:.6} {u:.6} {vv:.6} {}",
        v.weights.len()
    );
    for (bone, w) in &v.weights {
        let _ = write!(s, " {bone} {w:.6}");
    }
    s.push('\n');
}
