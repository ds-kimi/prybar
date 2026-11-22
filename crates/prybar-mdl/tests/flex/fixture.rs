//! Flex records patched into the synthetic v49 model.

#[path = "../common/mod.rs"]
mod common;

use common::put;

fn put_str(buf: &mut [u8], at: usize, s: &str) {
    buf[at..at + s.len()].copy_from_slice(s.as_bytes());
}

/// Desc, controller, rule `jaw_drop = c0 * 1.5`, one mesh flex moving
/// vertex 0 by +1 X (float16), one eyeball.
pub fn build() -> Vec<u8> {
    let mut b = common::build();
    let f = |v: f32| v.to_bits() as i32;
    #[rustfmt::skip]
    put(&mut b, &[
        (260, 1), (264, 1700), (1700, 60),
        (268, 1), (272, 1704), (1704, 66), (1708, 56), (1716, f(0.0)), (1720, f(1.0)),
        (276, 1), (280, 1724), (1724, 0), (1728, 3), (1732, 12),
        (1736, 2), (1740, 0), (1744, 1), (1748, f(1.5)), (1752, 6), (1756, 0),
        (860 + 16, 1), (860 + 20, 1780 - 860),
        (1780, 0), (1784, f(0.0)), (1788, f(1.0)), (1792, f(1.0)), (1796, f(1.0)),
        (1800, 1), (1804, 60), (1808, 0),
        (1840, 0x00ff_0000), (1844, 0x3c00),
        (712 + 100, 1), (712 + 104, 1860 - 712), (1860, 1770 - 1860), (1860 + 24, f(0.5)),
    ]);
    put_str(&mut b, 1760, "jaw_drop");
    put_str(&mut b, 1770, "phoneme");
    b
}
