//! A tiny synthetic v10 model: two bones (the root turned 90 degrees
//! about Z), one attachment, hitbox, 2x2 texture, body part, model with
//! one strip triangle, and one two-frame sequence with one event.

const SIZE: usize = 2048;
pub const BONES: usize = 256;
const ATT: usize = 480;
const HITBOX: usize = 568;
const TEX: usize = 600;
const SKINS: usize = 1452;
const PART: usize = 1456;
const MODEL: usize = 1532;
const MESH: usize = 1700;
const TRIS: usize = 1720;
const SEQ: usize = 1748;
const EVENT: usize = 1924;
const ANIM: usize = 2000;

pub fn put(buf: &mut [u8], at: usize, v: i32) {
    buf[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

pub fn float(buf: &mut [u8], at: usize, v: f32) {
    put(buf, at, v.to_bits() as i32);
}

pub fn text(buf: &mut [u8], at: usize, s: &str) {
    buf[at..at + s.len()].copy_from_slice(s.as_bytes());
}

/// Builds the model; header slots are word indices (offset = slot * 4).
pub fn build() -> Vec<u8> {
    let mut b = vec![0u8; SIZE];
    b[0..4].copy_from_slice(b"IDST");
    put(&mut b, 4, 10);
    text(&mut b, 8, "gold/box.mdl");
    float(&mut b, 19 * 4, 5.0);
    #[rustfmt::skip]
    let slots = [
        (35, 2), (36, BONES as i32), (39, 1), (40, HITBOX as i32), (41, 1),
        (42, SEQ as i32), (43, 1), (45, 1), (46, TEX as i32), (48, 1), (49, 1),
        (50, SKINS as i32), (51, 1), (52, PART as i32), (53, 1), (54, ATT as i32),
    ];
    for (slot, v) in slots {
        put(&mut b, slot * 4, v);
    }
    super::records::skeleton(&mut b, BONES, ATT, HITBOX);
    super::records::texture(&mut b, TEX);
    super::mesh_records::body(&mut b, (PART, MODEL, MESH, TRIS));
    super::records::sequence(&mut b, SEQ, EVENT, ANIM);
    b
}
