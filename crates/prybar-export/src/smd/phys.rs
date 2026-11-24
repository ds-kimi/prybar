use super::phys_piece;
use super::phys_space::{solid_pose, Space};
use super::skeleton;
use prybar_mdl::ir::{KvBlock, Physics};
use prybar_mdl::ModelIr;
use std::collections::HashMap;

/// Collision SMD from the loaded `.phy`, `None` without one. A single
/// solid is stored in model space; ragdoll solids are each stored in the
/// space of their bone and moved back to the bind pose here, unless the
/// solid carries its own reference pose (VTMB `origin`/`angles`).
pub fn physics_smd(ir: &ModelIr) -> Option<String> {
    let phy = ir.physics.as_ref()?;
    let mut s = String::from("version 1\n");
    skeleton::write(&mut s, &ir.bones);
    s.push_str("triangles\n");
    let ragdoll = phy.solids.len() > 1;
    let blocks = solid_blocks(phy);
    for (si, solid) in phy.solids.iter().enumerate() {
        let block = blocks.get(&si).copied();
        let bone = solid_bone(ir, phy, block.and_then(|b| b.get("name")), si);
        let bind = ir
            .bones
            .get(bone)
            .map(|b| b.pose_to_bone)
            .filter(|_| ragdoll);
        // Only VTMB writes `origin`/`angles` with this meaning.
        let pose = block.filter(|_| ir.version == 2531).and_then(solid_pose);
        let to_model = Space::pick(pose, bind);
        for piece in &solid.pieces {
            let tris: Vec<_> = piece
                .triangles
                .iter()
                .map(|t| t.map(|v| to_model.apply(v)))
                .collect();
            phys_piece::write(&mut s, bone, &tris);
        }
    }
    s.push_str("end\n");
    Some(s)
}

/// Solid index to text block from the text section, built once so the
/// lookup stays linear on files with many solids and blocks.
fn solid_blocks(phy: &Physics) -> HashMap<usize, &KvBlock> {
    let solids = phy.blocks.iter().filter(|b| b.name == "solid");
    solids
        .filter_map(|b| Some((b.get("index")?.parse().ok()?, b)))
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
