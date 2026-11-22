#![allow(dead_code, unused_imports)]
//! Tiny synthetic v49 model: one bone, one 3-frame animation, one
//! sequence with an event. The animation stream sits in the MDL, or in a
//! separate `.ani` block when `in_ani` is set.

pub const SIZE: usize = 4096;
pub const ANIM: usize = 700;
pub const DATA: usize = 1000;

mod parts;

pub use parts::ani;

pub fn put(buf: &mut [u8], at: usize, bytes: &[u8]) {
    buf[at..at + bytes.len()].copy_from_slice(bytes);
}

pub fn ints(buf: &mut [u8], pairs: &[(usize, i32)]) {
    for &(at, v) in pairs {
        put(buf, at, &v.to_le_bytes());
    }
}

pub fn build(in_ani: bool) -> Vec<u8> {
    let mut b = vec![0u8; SIZE];
    header(&mut b);
    if in_ani {
        ints(
            &mut b,
            &[
                (352, 2),
                (356, 1600),
                (1608, 16),
                (1612, 64),
                (ANIM + 52, 1),
                (ANIM + 56, 0),
            ],
        );
    } else {
        put(&mut b, DATA, &parts::stream());
    }
    parts::sequence(&mut b);
    b
}

fn header(b: &mut [u8]) {
    put(b, 0, b"IDST");
    #[rustfmt::skip]
    ints(b, &[
        (4, 49), (76, SIZE as i32), (156, 1), (160, 408),
        (180, 1), (184, ANIM as i32), (188, 1), (192, 1200),
        (408, 3000 - 408), (412, -1),
        (440, 1f32.to_bits() as i32), (444, 2f32.to_bits() as i32),
        (448, 3f32.to_bits() as i32), (464, 1f32.to_bits() as i32),
        (480, 1f32.to_bits() as i32), (484, 1f32.to_bits() as i32),
        (488, 1f32.to_bits() as i32),
        (ANIM + 4, 3000 + 10 - ANIM as i32), (ANIM + 8, 30f32.to_bits() as i32),
        (ANIM + 16, 3), (ANIM + 56, (DATA - ANIM) as i32),
    ]);
}
