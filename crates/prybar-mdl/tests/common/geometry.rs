//! VVD and VTX companions for the synthetic model: 8 vertices, one mesh,
//! two triangles in one triangle-list strip.

use super::put;

pub fn vvd() -> Vec<u8> {
    let mut buf = vec![0u8; 64 + 8 * 48];
    buf[0..4].copy_from_slice(b"IDSV");
    put(
        &mut buf,
        &[
            (4, 4),
            (8, 1234),
            (12, 1),
            (16, 8),
            (56, 64),
            (60, 64 + 8 * 48),
        ],
    );
    for i in 0..8 {
        let at = 64 + i * 48;
        buf[at..at + 4].copy_from_slice(&1f32.to_le_bytes());
        buf[at + 15] = 1;
        buf[at + 16..at + 20].copy_from_slice(&(i as f32).to_le_bytes());
    }
    buf
}

pub fn vtx() -> Vec<u8> {
    let mut buf = vec![0u8; 209];
    #[rustfmt::skip]
    put(&mut buf, &[
        (0, 7), (16, 1234), (20, 1), (28, 1), (32, 36),
        (36, 1), (40, 8), (44, 1), (48, 8), (52, 1), (56, 12),
        (64, 1), (68, 9), (73, 8), (77, 25), (81, 6), (85, 97),
        (89, 1), (93, 109), (182, 6),
    ]);
    buf[182 + 18] = 1;
    for i in 0..8u16 {
        let at = 98 + usize::from(i) * 9 + 4;
        buf[at..at + 2].copy_from_slice(&i.to_le_bytes());
    }
    for (n, i) in [0u16, 1, 2, 2, 1, 3].iter().enumerate() {
        buf[170 + n * 2..172 + n * 2].copy_from_slice(&i.to_le_bytes());
    }
    buf
}
