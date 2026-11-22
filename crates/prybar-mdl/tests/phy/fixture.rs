#![allow(dead_code)]
//! Tiny synthetic `.phy`: one solid, one ledge, one triangle.

pub const CHECKSUM: i32 = 7;
/// Offset of the single ledge, for tests that forge its fields.
pub const LEDGE: usize = 16 + 4 + 28 + 48;
pub const TEXT: &str = "solid {\n\"index\" \"0\"\n\"name\" \"root\"\n\"mass\" \"5.0\"\n}\n";

pub fn put(data: &mut [u8], at: usize, v: i32) {
    data[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

fn i32s(out: &mut Vec<u8>, v: &[i32]) {
    for x in v {
        out.extend_from_slice(&x.to_le_bytes());
    }
}

/// Ledge header, one triangle using points 0, 1, 2, then the points (meters).
fn ledge(out: &mut Vec<u8>) {
    i32s(out, &[32, 1, 0, 1]);
    i32s(out, &[0, 0, 1, 2]);
    let pts: [[f32; 4]; 3] = [[0.0; 4], [1.0, 0.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]];
    for p in pts.iter().flatten() {
        out.extend_from_slice(&p.to_le_bytes());
    }
}

pub fn build() -> Vec<u8> {
    let mut out = Vec::new();
    i32s(&mut out, &[16, 0, 1, CHECKSUM]);
    let mut solid = b"VPHY".to_vec();
    solid.extend_from_slice(&[0, 1, 0, 0]);
    solid.extend_from_slice(&[0; 20]);
    solid.extend_from_slice(&[0; 44]);
    solid.extend_from_slice(b"IVPS");
    ledge(&mut solid);
    i32s(&mut out, &[solid.len() as i32]);
    out.extend_from_slice(&solid);
    out.extend_from_slice(TEXT.as_bytes());
    out.push(0);
    out
}
