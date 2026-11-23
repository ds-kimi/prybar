//! A one-triangle VTX v107: one body part, model, LOD, mesh, strip group
//! and strip. Static-prop groups (`flags` 0x10) store bare u16 ids.

fn put16(buf: &mut [u8], at: usize, v: i16) {
    buf[at..at + 2].copy_from_slice(&v.to_le_bytes());
}

fn put32(buf: &mut [u8], at: usize, v: i32) {
    buf[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

pub fn vtx107(checksum: i32, flags: u8) -> Vec<u8> {
    let mut b = vec![0u8; 160];
    #[rustfmt::skip]
    let ints = [
        (0, 107), (16, checksum), (28, 1), (32, 36), (36, 1), (40, 8),
        (44, 1), (48, 8), (52, 1), (56, 12), (68, 8), (80, 20), (84, 56), (88, 64),
    ];
    for (at, v) in ints {
        put32(&mut b, at, v);
    }
    for (at, v) in [(64, 1), (72, 3), (74, 3), (76, 1), (136, 3)] {
        put16(&mut b, at, v);
    }
    b[78] = flags;
    b[145] = 1;
    for i in 0..3u8 {
        let at = if flags & 0x10 != 0 {
            92 + usize::from(i) * 2
        } else {
            102 + usize::from(i) * 12
        };
        b[at] = i;
        b[128 + usize::from(i) * 2] = i;
    }
    b
}
