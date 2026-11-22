//! Tiny synthetic v53 MDL: shifted header, one bone, one animdesc and
//! two embedded blobs standing in for the VVD and VTX.

const SIZE: usize = 1024;
const BONE: usize = 512;
const ANIM: usize = 768;

pub fn put(buf: &mut [u8], pairs: &[(usize, i32)]) {
    for &(at, v) in pairs {
        buf[at..at + 4].copy_from_slice(&v.to_le_bytes());
    }
}

fn put_str(buf: &mut [u8], at: usize, s: &str) {
    buf[at..at + s.len()].copy_from_slice(s.as_bytes());
}

pub fn build() -> Vec<u8> {
    let mut buf = vec![0u8; SIZE];
    buf[0..4].copy_from_slice(b"IDST");
    put_str(&mut buf, 16, "short.mdl");
    #[rustfmt::skip]
    put(&mut buf, &[
        (4, 53), (8, 77), (12, 860), (80, SIZE as i32),
        // bones, local anims: v49 offsets 156/180 plus the 4-byte shift.
        (160, 1), (164, BONE as i32), (184, 1), (188, ANIM as i32),
        // vtx at 960 (16 bytes), vvd at 980 (8 bytes), no phy.
        (428, 960), (432, 980), (444, 16), (448, 8),
    ]);
    bone(&mut buf);
    #[rustfmt::skip]
    put(&mut buf, &[
        (ANIM + 4, 940 - ANIM as i32), (ANIM + 8, 30f32.to_bits() as i32),
        (ANIM + 12, 1), (ANIM + 16, 12),
    ]);
    put_str(&mut buf, 860, "models/long/path/model.mdl");
    put_str(&mut buf, 940, "idle");
    buf
}

fn bone(buf: &mut [u8]) {
    let b = BONE as i32;
    #[rustfmt::skip]
    put(buf, &[
        (BONE, 920 - b), (BONE + 4, -1),
        // position.x, quat.w, rotation-scale x, then surfaceprop + contents.
        (BONE + 32, 2f32.to_bits() as i32), (BONE + 56, 1f32.to_bits() as i32),
        (BONE + 96, 5f32.to_bits() as i32),
        (BONE + 184, 1), (BONE + 200, 930 - b), (BONE + 204, 3),
    ]);
    put_str(buf, 920, "root");
    put_str(buf, 930, "metal");
}
