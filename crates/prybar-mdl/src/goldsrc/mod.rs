//! GoldSrc MDL v10 (Half-Life) and v14. Offsets are absolute and
//! vertices are stored in bone space, so geometry is moved to model
//! space here and the exporters treat it like any Source mesh.

mod anim;
mod body;
mod header;
mod materials;
mod seq;
mod skeleton;
mod skin;
mod transform;

use crate::read::Source;
use crate::{ModelIr, Result};
use header::{
    GoldHeader, BONE_CONTROLLERS, EYE, FLAGS, HULL_MAX, HULL_MIN, SEQ_GROUPS, VIEW_MAX, VIEW_MIN,
};

/// Files that complete a GoldSrc model: `<name>T.mdl` holds the textures
/// when the model was compiled with `$externaltextures`, and
/// `<name>01.mdl`, `<name>02.mdl`... hold sequence groups 1, 2...
#[derive(Debug, Clone, Copy, Default)]
pub struct Companions<'a> {
    pub textures: Option<&'a [u8]>,
    pub groups: &'a [&'a [u8]],
}

/// Parses a v10 or v14 model and its companions into the IR.
pub fn parse(data: &[u8], comp: Companions) -> Result<ModelIr> {
    let r = Source::new(data);
    let h = GoldHeader::parse(&r)?;
    let mut ir = base_ir(&h);
    ir.bones = skeleton::bones(&r, &h)?;
    ir.attachments = skeleton::attachments(&r, &h)?;
    ir.hitbox_sets = skeleton::hitbox_sets(&r, &h)?;
    let groups: Vec<Source> = comp.groups.iter().map(|g| Source::new(g)).collect();
    (ir.sequences, ir.animations) = seq::sequences(&r, &h, &ir.bones, &groups)?;
    if h.version == 10 {
        materials::load(&r, &h, comp.textures, &mut ir)?;
        let mats = transform::bone_matrices(&ir.bones);
        let dims = materials::slot_dims(&ir);
        let ctx = body::Ctx {
            r: &r,
            mats: &mats,
            dims: &dims,
        };
        ir.body_parts = body::body_parts(&ctx, &h)?;
    } else {
        ir.notes
            .push("meshes, textures: v14 layouts are not decoded".into());
    }
    notes(&h, groups.len(), &mut ir);
    Ok(ir)
}

fn base_ir(h: &GoldHeader) -> ModelIr {
    ModelIr {
        version: h.version,
        name: h.name.clone(),
        flags: h.w.i(FLAGS),
        eye_position: h.w.v3(EYE),
        hull_min: h.w.v3(HULL_MIN),
        hull_max: h.w.v3(HULL_MAX),
        view_min: h.w.v3(VIEW_MIN),
        view_max: h.w.v3(VIEW_MAX),
        ..ModelIr::default()
    }
}

/// Lists what a decompile of this file will be missing.
fn notes(h: &GoldHeader, groups: usize, ir: &mut ModelIr) {
    let controllers = h.w.i(BONE_CONTROLLERS);
    if controllers != 0 {
        ir.notes.push(format!(
            "bone controllers: {controllers} in file, not decoded"
        ));
    }
    let external = h.w.i(SEQ_GROUPS).saturating_sub(1);
    if external > 0 && (groups as i64) < i64::from(external) {
        ir.notes.push(format!(
            "sequence groups: {external} external, frames need their NN.mdl files"
        ));
    }
}
