//! QC compile script: everything studiomdl needs to rebuild the model
//! from the SMD, VTA and VRD files written next to it.

mod anim;
mod flex;
mod fmt;
mod model;
mod names;
mod physics;
mod sanitize;
mod skeleton;

use crate::smd::{mesh_files, physics_smd, SmdFile};
pub use names::Names;
use prybar_mdl::ModelIr;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct QcOptions {
    /// One `.qci` include per section instead of a single QC, the layout
    /// Valve uses for big models so sections can be shared or swapped.
    pub split: bool,
}

/// The QC, plus its includes when split; empty sections are left out.
pub fn qc_files(ir: &ModelIr, names: &Names, opts: QcOptions) -> Vec<SmdFile> {
    let parts = [
        ("model", model::write(ir, names)),
        ("bones", skeleton::write(ir, names)),
        ("animations", anim::write(ir, names)),
        ("collision", physics::write(ir, names)),
    ];
    let parts = parts.into_iter().filter(|(_, t)| !t.is_empty());
    let main = format!("{}.qc", names.stem);
    if !opts.split {
        let text = parts.map(|(_, t)| t).collect::<Vec<_>>().join("\n");
        return vec![SmdFile { name: main, text }];
    }
    let mut out = vec![SmdFile {
        name: main,
        text: String::new(),
    }];
    for (section, text) in parts {
        let name = format!("{}_{section}.qci", names.stem);
        out[0]
            .text
            .push_str(&format!("$include {}\n", fmt::quote(&name)));
        out.push(SmdFile { name, text });
    }
    out
}

/// Every file of a decompile (meshes, anims, VTA, collision, VRD, QC)
/// with names that agree with each other.
pub fn decompile_files(ir: &ModelIr, opts: QcOptions) -> Vec<SmdFile> {
    let names = Names::new(ir);
    let mut out = mesh_files(ir);
    out.extend(crate::vta::vta_files(ir));
    let extra = [
        (names.physics.clone(), physics_smd(ir)),
        (names.vrd.clone(), crate::vrd::vrd(ir)),
    ];
    for (name, text) in extra {
        if let (Some(name), Some(text)) = (name, text) {
            out.push(SmdFile { name, text });
        }
    }
    out.extend(qc_files(ir, &names, opts));
    out
}
