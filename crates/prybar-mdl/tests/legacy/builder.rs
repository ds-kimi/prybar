//! Tiny synthetic legacy MDLs: one bone, hitbox, texture, body part with
//! one model and, where the version has them, one attachment.

const SIZE: usize = 2048;
use super::records::{records, ATT, BONE, PART, SET, TEX};

/// Header layout the builder writes, mirroring the parser's slots.
struct Shape {
    name_len: usize,
    vectors: usize,
    /// Slots of bones, hitbox sets, anims, textures, body parts, attachments.
    slots: [usize; 6],
    model_name: usize,
}

fn shape(version: i32) -> Shape {
    match version {
        2531 => Shape {
            name_len: 128,
            vectors: 7,
            slots: [3, 7, 9, 16, 23, 25],
            model_name: 128,
        },
        27..=31 => Shape {
            name_len: 64,
            vectors: 6,
            slots: [1, 7, 9, 20, 27, 29],
            model_name: 64,
        },
        _ => Shape {
            name_len: 64,
            vectors: 6,
            slots: [1, 5, 7, 18, 25, 27],
            model_name: 64,
        },
    }
}

pub fn put(buf: &mut [u8], at: usize, v: i32) {
    buf[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

pub fn put_str(buf: &mut [u8], at: usize, s: &str) {
    buf[at..at + s.len()].copy_from_slice(s.as_bytes());
}

/// Builds a model of the given version; returns it with the offset of the
/// header int run so tests can poke slots.
pub fn build(version: i32) -> (Vec<u8>, usize) {
    let sh = shape(version);
    let mut buf = vec![0u8; SIZE];
    buf[0..4].copy_from_slice(b"IDST");
    put(&mut buf, 4, version);
    put(&mut buf, 8, 99);
    put_str(&mut buf, 12, "legacy/box.mdl");
    put(&mut buf, 16 + sh.name_len, 7f32.to_bits() as i32);
    let ints = 16 + sh.name_len + sh.vectors * 12;
    let [bones, sets, anims, tex, parts, att] = sh.slots;
    #[rustfmt::skip]
    let pairs = [
        (bones, 1), (bones + 1, BONE as i32), (sets, 1), (sets + 1, SET as i32),
        (anims, 3), (tex, 1), (tex + 1, TEX as i32), (parts, 1),
        (parts + 1, PART as i32), (att, 1), (att + 1, ATT as i32),
    ];
    for (s, v) in pairs {
        put(&mut buf, ints + s * 4, v);
    }
    records(&mut buf, sh.model_name);
    (buf, ints)
}
