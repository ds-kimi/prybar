//! MDL 2531 and v27-v37: the pre-release Source formats. They predate the
//! VVD/VTX split and many of their structs are only partly known, so the
//! parser recovers the skeleton, markers, materials and body part names
//! (plus meshes, vertices, flexes, eyeballs, pose parameters and
//! procedural bones for 2531), and lists everything else in
//! [`ModelIr::notes`].

mod body;
mod bones;
mod face;
mod geometry;
mod header;
mod layout;
mod markers;

use crate::read::Source;
use crate::{ModelIr, Result};
use header::LegacyHeader;
use layout::Layout;

pub fn parse(data: &[u8]) -> Result<ModelIr> {
    let r = Source::new(data);
    let version: i32 = r.read(4)?;
    let lay = layout::for_version(version);
    let hdr = LegacyHeader::parse(&r, &lay)?;
    let mut ir = base_ir(&r, &hdr, &lay)?;
    ir.bones = bones::bones(&r, &hdr, &lay)?;
    ir.attachments = markers::attachments(&r, &hdr, &lay)?;
    ir.hitbox_sets = markers::hitbox_sets(&r, &hdr, &lay)?;
    ir.materials = body::materials(&r, &hdr, &lay)?;
    ir.skins = body::skins(&r, &hdr, &lay)?;
    ir.body_parts = body::body_parts(&r, &hdr, &lay)?;
    ir.notes = notes(&hdr, &lay);
    if lay.inline_vertices {
        face::tables(&r, &hdr, &mut ir)?;
        ir.pose_params = crate::seq::pose_param_table(&r, hdr.span(lay.pose_params))?;
        geometry::attach(&r, &hdr, &lay, &mut ir)?;
    }
    Ok(ir)
}

fn base_ir(r: &Source, hdr: &LegacyHeader, lay: &Layout) -> Result<ModelIr> {
    let [eye, illum, hull_min, hull_max, view_min, view_max] = hdr.vectors;
    let float = |slot: Option<usize>| slot.map_or(0.0, |s| f32::from_bits(hdr.int(s) as u32));
    Ok(ModelIr {
        version: hdr.version,
        name: hdr.name.clone(),
        checksum: hdr.checksum,
        flags: hdr.int(lay.flags),
        eye_position: eye,
        illum_position: illum,
        hull_min,
        hull_max,
        view_min,
        view_max,
        mass: float(lay.mass),
        surface_prop: crate::read::name_at(r, 0, hdr.int(lay.surface_prop))?,
        ..ModelIr::default()
    })
}

/// Names each section the file has but the parser leaves out, so callers
/// see exactly what a decompile of this version will be missing.
fn notes(hdr: &LegacyHeader, lay: &Layout) -> Vec<String> {
    let mut out = Vec::new();
    if !lay.inline_vertices {
        out.push("meshes: legacy vertex data is not decoded".to_string());
    }
    let sections = [
        (lay.anims, "animations"),
        (lay.seqs, "sequences"),
        (lay.flex_ctrls, "flex controllers"),
        (lay.pose_params, "pose parameters"),
    ];
    for (slot, what) in sections {
        // 2531 decodes its flex tables (see `face`) and pose parameters.
        if lay.inline_vertices && (slot == lay.flex_ctrls || slot == lay.pose_params) {
            continue;
        }
        let n = hdr.int(slot);
        if n != 0 {
            out.push(format!("{what}: {n} in file, not decoded"));
        }
    }
    if lay.attachments.is_none() {
        out.push("attachments: layout unknown for this version".into());
    }
    out
}
