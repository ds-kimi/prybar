use super::phys_math::bone_to_model;
use super::phys_piece;
use super::skeleton;
use prybar_mdl::ir::Physics;
use prybar_mdl::ModelIr;
use std::collections::HashMap;

/// Collision SMD from the loaded `.phy`, `None` without one. A single
/// solid is stored in model space; ragdoll solids are each stored in the
/// space of their bone and moved back to the bind pose here.
pub fn physics_smd(ir: &ModelIr) -> Option<String> {
    let phy = ir.physics.as_ref()?;
    let mut s = String::from("version 1\n");
    skeleton::write(&mut s, &ir.bones);
    s.push_str("triangles\n");
    let ragdoll = phy.solids.len() > 1;
    let names = solid_names(phy);
    for (si, solid) in phy.solids.iter().enumerate() {
        let bone = solid_bone(ir, phy, names.get(&si).copied(), si);
        let pose = ir.bones.get(bone).map(|b| &b.pose_to_bone);
        for piece in &solid.pieces {
            let tris: Vec<_> = match pose.filter(|_| ragdoll) {
                Some(m) => piece
                    .triangles
                    .iter()
                    .map(|t| t.map(|v| bone_to_model(m, v)))
                    .collect(),
                None => piece.triangles.clone(),
            };
            phys_piece::write(&mut s, bone, &tris);
        }
    }
    s.push_str("end\n");
    Some(s)
}

/// Solid index to bone name from the text section, built once so the
/// lookup stays linear on files with many solids and blocks.
fn solid_names(phy: &Physics) -> HashMap<usize, &str> {
    let solids = phy.blocks.iter().filter(|b| b.name == "solid");
    solids
        .filter_map(|b| Some((b.get("index")?.parse().ok()?, b.get("name")?)))
        .collect()
}

/// The ledge client data is a fallback for files whose names do not
/// match a bone (renamed bones).
fn solid_bone(ir: &ModelIr, phy: &Physics, name: Option<&str>, solid: usize) -> usize {
    let by_name = name.and_then(|n| ir.bones.iter().position(|b| b.name.eq_ignore_ascii_case(n)));
    let by_client = phy.solids[solid]
        .pieces
        .first()
        .and_then(|p| usize::try_from(p.bone).ok())
        .filter(|&b| b < ir.bones.len());
    by_name.or(by_client).unwrap_or(0)
}
