//! MDL 2531 models with one mesh of inline vertices.

use super::builder::{build, put};
use prybar_mdl::{parse, ModelIr};

/// The one model sits right after its body part record.
pub const MODEL: usize = 1116;
pub const MESH: usize = 2100;
const VERTS: usize = 2200;

/// A 2531 model whose only mesh holds `verts`, stored as `kind` with
/// position scale 2 and offset 0.
pub fn model(kind: i32, stride: usize, verts: &[Vec<u8>]) -> ModelIr {
    parse(&buffer(kind, stride, verts).0).unwrap()
}

/// The raw file of [`model`], with the header int run offset.
pub fn buffer(kind: i32, stride: usize, verts: &[Vec<u8>]) -> (Vec<u8>, usize) {
    let (mut buf, ints) = build(2531);
    buf.resize(4096, 0);
    let f = MODEL + 128;
    let n = verts.len() as i32;
    let rel = |at: usize| (at - MODEL) as i32;
    for (i, v) in [(2, 1), (3, rel(MESH)), (4, n), (5, rel(VERTS)), (7, kind)] {
        put(&mut buf, f + i * 4, v);
    }
    for i in 11..14 {
        put(&mut buf, f + i * 4, 2f32.to_bits() as i32);
    }
    put(&mut buf, MESH + 8, n);
    for (k, v) in verts.iter().enumerate() {
        buf[VERTS + k * stride..][..v.len()].copy_from_slice(v);
    }
    (buf, ints)
}

pub fn skinned(w: [u8; 4], pos: f32) -> Vec<u8> {
    let mut v = w.to_vec();
    for b in [1i16, 2, 3, 0] {
        v.extend(b.to_le_bytes());
    }
    for f in [pos, 0.0, 0.0, 0.0, 0.0, 1.0, 0.25, 0.75] {
        v.extend(f.to_le_bytes());
    }
    v
}
