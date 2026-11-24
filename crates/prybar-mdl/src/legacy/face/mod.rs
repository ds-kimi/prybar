//! MDL 2531 facial data: flex descs, controllers and rules keep the v44
//! record shapes, while eyeballs, mesh flexes and vertex animations use
//! their own smaller layouts.

mod deltas;

pub use deltas::mesh_flexes;

use super::header::LegacyHeader;
use crate::ir::Eyeball;
use crate::read::{name_at, table, Source, Words};
use crate::{ModelIr, Result};

/// Header slots: phoneme filter floats, then the flex desc, controller and
/// rule `(count, index)` pairs.
const PHONEME_FILTER: usize = 1;
const FLEX_DESCS: usize = 29;
const FLEX_CONTROLLERS: usize = 31;
const FLEX_RULES: usize = 33;

/// `mstudioeyeball_t` of 2531: 35 words, no unused tail.
const EYEBALL_STRIDE: usize = 140;

pub fn tables(r: &Source, hdr: &LegacyHeader, ir: &mut ModelIr) -> Result<()> {
    let float = |slot: usize| f32::from_bits(hdr.int(slot) as u32);
    ir.phoneme_filter = [float(PHONEME_FILTER), float(PHONEME_FILTER + 1)];
    let spans = [
        hdr.span(FLEX_DESCS),
        hdr.span(FLEX_CONTROLLERS),
        hdr.span(FLEX_RULES),
    ];
    crate::flex::tables(r, spans, ir)
}

/// The eyeball record has no material; the engine binds eye `i` to every
/// mesh whose material type is 1 and parameter is `i`, so the texture is
/// taken from that mesh. `meshes` holds `(type, param, material)`.
pub fn eyeballs(
    r: &Source,
    model_at: usize,
    span: (i32, i32),
    meshes: &[(i32, i32, i32)],
) -> Result<Vec<Eyeball>> {
    let mut out = Vec::new();
    for (i, at) in table(r, model_at, span.0, span.1, EYEBALL_STRIDE)?
        .into_iter()
        .enumerate()
    {
        let mut eye = eyeball(r, at)?;
        let index = i32::try_from(i).unwrap_or(-1);
        if let Some(m) = meshes.iter().find(|m| m.0 == 1 && m.1 == index) {
            eye.texture = m.2;
        }
        out.push(eye);
    }
    Ok(out)
}

fn eyeball(r: &Source, at: usize) -> Result<Eyeball> {
    let w = Words::read(r, at, EYEBALL_STRIDE / 4)?;
    let three = |n: usize| [w.i(n), w.i(n + 1), w.i(n + 2)];
    Ok(Eyeball {
        name: name_at(r, at, w.i(0))?,
        bone: w.i(1),
        origin: w.v3(2),
        z_offset: w.f(5),
        radius: w.f(6),
        up: w.v3(7),
        forward: w.v3(10),
        texture: w.i(13),
        iris_scale: w.f(15),
        upper_flex_descs: three(17),
        lower_flex_descs: three(20),
        upper_targets: w.v3(23),
        lower_targets: w.v3(26),
        upper_lid_flex_desc: w.i(29),
        lower_lid_flex_desc: w.i(30),
        non_facs: false,
    })
}
