use prybar_mdl::ir::{Flex, SubModel};
use prybar_mdl::ModelIr;
use std::collections::HashMap;

/// One VTA time step: the same flex desc and targets on every mesh of the
/// sub-model, with the first vertex of each mesh it touches.
pub struct Frame<'a> {
    pub label: String,
    pub parts: Vec<(usize, &'a Flex)>,
}

/// studiomdl splits one VTA frame into per-mesh flexes, so meshes that
/// share desc, partner and targets go back into a single frame.
pub fn frames<'a>(ir: &ModelIr, model: &'a SubModel) -> Vec<Frame<'a>> {
    let mut out: Vec<Frame> = Vec::new();
    let mut seen = HashMap::new();
    for mesh in &model.meshes {
        let base = usize::try_from(mesh.vertex_offset).unwrap_or(0);
        for f in &mesh.flexes {
            let key = (f.desc, f.partner, f.targets.map(f32::to_bits));
            let i = *seen.entry(key).or_insert_with(|| {
                out.push(Frame {
                    label: label(ir, f),
                    parts: Vec::new(),
                });
                out.len() - 1
            });
            out[i].parts.push((base, f));
        }
    }
    out
}

/// A stereo flex is labelled `left+right`, as Crowbar-era tools expect.
fn label(ir: &ModelIr, f: &Flex) -> String {
    let name = |i: i32| {
        let raw = usize::try_from(i)
            .ok()
            .and_then(|i| ir.flex_descs.get(i))
            .map_or_else(|| format!("flex{i}"), Clone::clone);
        raw.chars().filter(|c| !c.is_control()).collect::<String>()
    };
    match f.partner {
        p if p > 0 => format!("{}+{}", name(f.desc), name(p)),
        _ => name(f.desc),
    }
}
