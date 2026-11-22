use crate::smd::names::{stem, unique};
use crate::smd::{anim_names, mesh_entries};
use prybar_mdl::ModelIr;
use std::collections::HashSet;

/// Every file name the QC refers to, built with the same collision rules
/// as the SMD writers so each reference matches a written file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Names {
    pub stem: String,
    /// `refs[part][model][lod]`; empty for blank sub-models.
    pub refs: Vec<Vec<Vec<String>>>,
    /// Per animation; `None` when its frames were not decoded.
    pub anims: Vec<Option<String>>,
    pub physics: Option<String>,
    pub vrd: Option<String>,
}

impl Names {
    pub fn new(ir: &ModelIr) -> Self {
        let mut taken = HashSet::new();
        let mut flat = mesh_entries(ir, &mut taken).into_iter().map(|e| e.2);
        let refs = ir.body_parts.iter().map(|p| {
            let models = p.models.iter();
            models
                .map(|m| flat.by_ref().take(m.lods.len()).collect())
                .collect()
        });
        let refs = refs.collect();
        let mut anims = vec![None; ir.animations.len()];
        for (i, name) in anim_names(ir, &mut taken) {
            anims[i] = Some(name);
        }
        let base = stem(ir.name.trim_end_matches(".mdl"), "model");
        let physics = ir
            .physics
            .as_ref()
            .map(|_| unique(&mut taken, format!("{base}_physics.smd")));
        let vrd = crate::vrd::vrd(ir).map(|_| format!("{base}.vrd"));
        Self {
            stem: base,
            refs,
            anims,
            physics,
            vrd,
        }
    }

    pub fn reference(&self, part: usize, model: usize, lod: usize) -> Option<&str> {
        let lods = self.refs.get(part)?.get(model)?;
        lods.get(lod).map(String::as_str)
    }

    /// VTA files are named after the reference SMD (see `vta_files`).
    pub fn vta(&self, part: usize, model: usize) -> Option<String> {
        let r = self.reference(part, model, 0)?;
        Some(format!("{}.vta", r.trim_end_matches(".smd")))
    }
}
