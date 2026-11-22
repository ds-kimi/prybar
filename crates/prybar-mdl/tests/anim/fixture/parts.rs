use super::{ints, put};

/// One record for bone 0: raw identity rotation (Quaternion48), then an
/// RLE x-position stream holding 10, 20 and a repeat of 20.
pub fn stream() -> Vec<u8> {
    let mut s = vec![0u8, 0x06, 0, 0];
    s.extend([0x00, 0x80, 0x00, 0x80, 0x00, 0x40]);
    s.extend([6, 0, 0, 0, 0, 0]);
    s.extend([2, 3, 10, 0, 20, 0]);
    s
}

pub fn sequence(b: &mut [u8]) {
    #[rustfmt::skip]
    ints(b, &[
        (1204, 3020 - 1200), (1208, 3030 - 1200), (1224, 1), (1228, 300),
        (1260, 250), (1268, 1), (1272, 1),
        (1500, 0.5f32.to_bits() as i32), (1504, 7), (1576, 3040 - 1500),
    ]);
    put(b, 1512, b"foo");
    for (at, s) in [
        (3000, "root"),
        (3010, "@walk"),
        (3020, "walk"),
        (3030, "ACT_WALK"),
        (3040, "AE_FOO"),
    ] {
        put(b, at, s.as_bytes());
    }
}

pub fn ani() -> Vec<u8> {
    let mut a = vec![0u8; 64];
    put(&mut a, 16, &stream());
    a
}
