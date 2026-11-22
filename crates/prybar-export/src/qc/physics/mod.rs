//! `$collisionmodel` (one solid) or `$collisionjoints` (ragdoll) from the
//! `.phy` text section.

mod joints;

use super::fmt::{quote, text_num};
use super::Names;
use prybar_mdl::ir::Physics;
use prybar_mdl::ModelIr;
use std::fmt::Write;

pub fn write(ir: &ModelIr, names: &Names) -> String {
    let mut s = String::new();
    let (Some(phy), Some(file)) = (&ir.physics, &names.physics) else {
        return s;
    };
    let ragdoll = phy.solids.len() > 1;
    let cmd = if ragdoll {
        "$collisionjoints"
    } else {
        "$collisionmodel"
    };
    let _ = writeln!(s, "{cmd} {} {{", quote(file));
    edit_params(&mut s, phy);
    if ragdoll {
        joints::write(&mut s, phy);
    } else {
        joints::single(&mut s, phy);
    }
    s.push_str("}\n");
    s
}

fn edit_params(s: &mut String, phy: &Physics) {
    let edit = phy.blocks.iter().find(|b| b.name == "editparams");
    let get = |k: &str| edit.and_then(|b| b.get(k)).filter(|v| !v.is_empty());
    if let Some(mass) = get("totalmass") {
        let _ = writeln!(s, "  $mass {}", text_num(mass));
    }
    let concave = phy.solids.iter().any(|x| x.pieces.len() > 1);
    if concave || get("concave") == Some("1") {
        s.push_str("  $concave\n");
    }
    // Without it studiomdl wraps each ragdoll bone's pieces in one convex
    // hull (Portal 2 doors and buttons lose their openings).
    if concave && phy.solids.len() > 1 {
        s.push_str("  $concaveperjoint\n");
    }
    if let Some(root) = get("rootname") {
        let _ = writeln!(s, "  $rootbone {}", quote(root));
    }
    let rules = phy.blocks.iter().find(|b| b.name == "collisionrules");
    if rules.and_then(|b| b.get("selfcollisions")) == Some("0") {
        s.push_str("  $noselfcollisions\n");
    }
}
