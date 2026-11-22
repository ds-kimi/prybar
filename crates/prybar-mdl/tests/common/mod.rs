#![allow(dead_code)]
//! Builds a tiny synthetic v49 MDL: one of every header-level record.

pub mod geometry;

const SIZE: usize = 2048;

pub fn put(buf: &mut [u8], pairs: &[(usize, i32)]) {
    for &(at, v) in pairs {
        buf[at..at + 4].copy_from_slice(&v.to_le_bytes());
    }
}

fn put_strs(buf: &mut [u8], strs: &[(usize, &str)]) {
    for &(at, s) in strs {
        buf[at..at + s.len()].copy_from_slice(s.as_bytes());
    }
}

fn header(buf: &mut [u8]) {
    buf[0..4].copy_from_slice(b"IDST");
    put_strs(buf, &[(12, "test/box.mdl")]);
    #[rustfmt::skip]
    put(buf, &[
        (4, 49), (8, 1234), (76, SIZE as i32),
        (156, 1), (160, 408), (172, 1), (176, 976),
        (204, 1), (208, 624), (212, 1), (216, 688),
        (220, 1), (224, 2), (228, 692), (232, 1), (236, 696),
        (240, 1), (244, 1056), (308, 1580), (312, 1600), (316, 13),
    ]);
}

pub fn build() -> Vec<u8> {
    let mut buf = vec![0u8; SIZE];
    header(&mut buf);
    #[rustfmt::skip]
    put(&mut buf, &[
        (408, 1500 - 408), (412, -1), (408 + 176, 1510 - 408),
        (624, 1520 - 624), (688, 1530), (692, 0x0001_0000),
        (696, 1550 - 696), (700, 1), (704, 1), (708, 16),
        (712 + 72, 1), (712 + 76, 148), (712 + 80, 8),
        (860 + 8, 8), (976, 1560 - 976), (980, 1), (984, 12),
        (988 + 32, 1570 - 988), (1056, 1575 - 1056),
    ]);
    put_strs(
        &mut buf,
        &[
            (712, "box"),
            (1500, "root"),
            (1510, "metal"),
            (1520, "box"),
            (1530, "models/test/"),
            (1550, "body"),
            (1560, "default"),
            (1570, "head"),
            (1575, "tip"),
            (1580, "flesh"),
            (1600, "mdlkeyvalue{}"),
        ],
    );
    buf
}
