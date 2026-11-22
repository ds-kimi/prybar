//! MDL v44-v53: every version in this range shares the same table layout,
//! so one parser covers them all. v52 differs only in small structs that
//! the shared readers branch on; v53 swaps in its own bones and animdescs.

mod assets;
mod body;
mod skeleton;

use crate::header::{fields, Header};
use crate::read::Source;
use crate::{ModelIr, Result};

pub fn parse(data: &[u8], ani: Option<&[u8]>) -> Result<ModelIr> {
    let r = Source::new(data);
    let hdr = Header::parse(&r)?;
    let mut ir = base_ir(&hdr);
    ir.bones = if hdr.version == 53 {
        crate::v53::bones(&r, &hdr)?
    } else {
        skeleton::bones(&r, &hdr)?
    };
    ir.attachments = skeleton::attachments(&r, &hdr)?;
    ir.hitbox_sets = skeleton::hitbox_sets(&r, &hdr)?;
    ir.ik_chains = skeleton::ik_chains(&r, &hdr)?;
    ir.ik_auto_locks = skeleton::ik_auto_locks(&r, &hdr)?;
    ir.body_parts = body::body_parts(&r, &hdr)?;
    assets::fill(&r, &hdr, &mut ir)?;
    crate::flex::fill(&r, &hdr, &mut ir)?;
    ir.pose_params = crate::seq::pose_params(&r, &hdr)?;
    ir.animations = if hdr.version == 53 {
        crate::v53::animations(&r, &hdr)?
    } else {
        crate::anim::animations(&r, &hdr, &ir.bones, ani)?
    };
    ir.sequences = crate::seq::sequences(&r, &hdr, ir.bones.len())?;
    Ok(ir)
}

fn base_ir(hdr: &Header) -> ModelIr {
    let [eye, illum, hull_min, hull_max, view_min, view_max] = hdr.vectors;
    ModelIr {
        version: hdr.version,
        name: hdr.name.clone(),
        checksum: hdr.checksum,
        flags: hdr.int(fields::FLAGS),
        eye_position: eye,
        illum_position: illum,
        hull_min,
        hull_max,
        view_min,
        view_max,
        mass: hdr.float(fields::MASS),
        ..ModelIr::default()
    }
}
