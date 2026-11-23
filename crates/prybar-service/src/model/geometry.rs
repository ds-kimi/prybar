use crate::source::Origin;
use prybar_mdl::ModelIr;

/// studiomdl writes one VTX per renderer; any of them holds the same
/// strips, dx90 is the one every modern game ships.
const VTX: [&str; 4] = [".dx90.vtx", ".vtx", ".dx80.vtx", ".sw.vtx"];

/// Vampire: Bloodlines ships dx80 and dx7 strips only, in VTX v107.
const VTX_2531: [&str; 3] = [".dx80.vtx", ".dx7_2bone.vtx", ".vtx"];

/// MDL 2531 carries its own vertices; only the triangles are missing.
pub fn attach_v2531(origin: Origin, stem: &str, ir: &mut ModelIr) {
    let vtx = VTX_2531
        .iter()
        .find_map(|s| origin.get(stem, s).ok().flatten());
    let done = match vtx {
        None => Err("no .vtx: meshes not exported".to_string()),
        Some(v) => prybar_mdl::parse_vtx(&v)
            .map_err(|e| format!(".vtx ignored: {e}"))
            .and_then(|v| {
                prybar_mdl::load_triangles(ir, &v).map_err(|e| format!("meshes not exported: {e}"))
            }),
    };
    if let Err(note) = done {
        ir.notes.push(note);
    }
}

pub fn attach(origin: Origin, stem: &str, ir: &mut ModelIr) {
    if let Err(note) = mesh(origin, stem, ir) {
        ir.notes.push(note);
    }
    match origin.get(stem, ".phy") {
        Ok(Some(phy)) if !phy.is_empty() => match prybar_mdl::parse_phy(&phy) {
            Ok(p) => ir.physics = Some(p),
            Err(e) => ir.notes.push(format!(".phy ignored: {e}")),
        },
        Ok(_) => {}
        Err(e) => ir.notes.push(format!(".phy ignored: {e}")),
    }
}

fn mesh(origin: Origin, stem: &str, ir: &mut ModelIr) -> Result<(), String> {
    let get = |suffix: &str| origin.get(stem, suffix).ok().flatten();
    let vvd = get(".vvd").ok_or("no .vvd: meshes not exported")?;
    let vtx = VTX
        .iter()
        .find_map(|s| get(s))
        .ok_or("no .vtx: meshes not exported")?;
    let vvd = prybar_mdl::parse_vvd(&vvd).map_err(|e| format!(".vvd ignored: {e}"))?;
    let vtx = prybar_mdl::parse_vtx(&vtx).map_err(|e| format!(".vtx ignored: {e}"))?;
    prybar_mdl::load_geometry(ir, &vvd, &vtx).map_err(|e| format!("meshes not exported: {e}"))
}
