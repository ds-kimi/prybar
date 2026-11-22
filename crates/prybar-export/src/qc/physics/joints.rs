use crate::qc::fmt::{quote, text_num};
use prybar_mdl::ir::{KvBlock, Physics};
use std::collections::HashMap;
use std::fmt::Write;

pub fn single(s: &mut String, phy: &Physics) {
    let Some(solid) = phy.blocks.iter().find(|b| b.name == "solid") else {
        return;
    };
    for (key, cmd) in [
        ("inertia", "$inertia"),
        ("damping", "$damping"),
        ("rotdamping", "$rotdamping"),
    ] {
        if let Some(v) = solid.get(key) {
            let _ = writeln!(s, "  {cmd} {}", text_num(v));
        }
    }
}

pub fn write(s: &mut String, phy: &Physics) {
    let names = solid_names(phy);
    for solid in phy.blocks.iter().filter(|b| b.name == "solid") {
        let Some(name) = solid.get("name") else {
            continue;
        };
        let pairs = [
            ("damping", "$jointdamping"),
            ("rotdamping", "$jointrotdamping"),
            ("inertia", "$jointinertia"),
        ];
        for (key, cmd) in pairs {
            if let Some(v) = solid.get(key) {
                let _ = writeln!(s, "  {cmd} {} {}", quote(name), text_num(v));
            }
        }
    }
    for c in phy.blocks.iter().filter(|b| b.name == "ragdollconstraint") {
        constraint(s, c, &names);
    }
}

fn constraint(s: &mut String, c: &KvBlock, names: &HashMap<&str, &str>) {
    let Some(child) = c.get("child").and_then(|i| names.get(i)) else {
        return;
    };
    for axis in ["x", "y", "z"] {
        let get = |k: &str| text_num(c.get(&format!("{axis}{k}")).unwrap_or("0"));
        let _ = writeln!(
            s,
            "  $jointconstrain {} {axis} limit {} {} {}",
            quote(child),
            get("min"),
            get("max"),
            get("friction")
        );
    }
}

fn solid_names(phy: &Physics) -> HashMap<&str, &str> {
    let solids = phy.blocks.iter().filter(|b| b.name == "solid");
    solids
        .filter_map(|b| Some((b.get("index")?, b.get("name")?)))
        .collect()
}
