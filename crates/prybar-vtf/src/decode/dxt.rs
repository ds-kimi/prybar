//! S3TC block decoding (DXT1, DXT3, DXT5).

#[derive(Clone, Copy)]
pub enum Alpha {
    None,
    Explicit,
    Interpolated,
}

pub fn expand565(c: u16) -> [u8; 3] {
    let (r, g, b) = ((c >> 11) & 31, (c >> 5) & 63, c & 31);
    [
        ((r << 3) | (r >> 2)) as u8,
        ((g << 2) | (g >> 4)) as u8,
        ((b << 3) | (b >> 2)) as u8,
    ]
}

/// Walks 4x4 blocks; pixels past the image edge are decoded and dropped.
pub fn blocks(data: &[u8], w: usize, h: usize, size: usize, out: &mut [u8], alpha: Alpha) {
    let bw = w.div_ceil(4);
    for (i, block) in data.chunks_exact(size).enumerate() {
        let (bx, by) = (i % bw * 4, i / bw * 4);
        let (a, c) = block.split_at(size - 8);
        let colors = palette(c, matches!(alpha, Alpha::None));
        let alphas = alpha_values(a, alpha);
        let idx = u32::from_le_bytes([c[4], c[5], c[6], c[7]]);
        for p in 0..16 {
            let (x, y) = (bx + p % 4, by + p / 4);
            if x >= w || y >= h {
                continue;
            }
            let rgba = colors[(idx >> (p * 2) & 3) as usize];
            let at = (y * w + x) * 4;
            out[at..at + 3].copy_from_slice(&rgba[..3]);
            out[at + 3] = alphas.map_or(rgba[3], |v| v[p]);
        }
    }
}

/// The four colors of a block; DXT1 with c0 <= c1 has a transparent black.
fn palette(c: &[u8], one_bit: bool) -> [[u8; 4]; 4] {
    let (c0, c1) = (
        u16::from_le_bytes([c[0], c[1]]),
        u16::from_le_bytes([c[2], c[3]]),
    );
    let (a, b) = (expand565(c0), expand565(c1));
    let mix = |wa: u16, wb: u16, d: u16| -> [u8; 4] {
        let ch = |i: usize| ((a[i] as u16 * wa + b[i] as u16 * wb) / d) as u8;
        [ch(0), ch(1), ch(2), 255]
    };
    let (p0, p1) = ([a[0], a[1], a[2], 255], [b[0], b[1], b[2], 255]);
    if c0 > c1 || !one_bit {
        [p0, p1, mix(2, 1, 3), mix(1, 2, 3)]
    } else {
        [p0, p1, mix(1, 1, 2), [0, 0, 0, 0]]
    }
}

fn alpha_values(a: &[u8], kind: Alpha) -> Option<[u8; 16]> {
    let mut v = [255u8; 16];
    match kind {
        Alpha::None => return None,
        Alpha::Explicit => (0..16).for_each(|p| v[p] = (a[p / 2] >> (p % 2 * 4) & 15) * 17),
        Alpha::Interpolated => {
            let (a0, a1) = (a[0] as u16, a[1] as u16);
            let bits = a[2..8]
                .iter()
                .rev()
                .fold(0u64, |acc, &b| acc << 8 | b as u64);
            for (p, slot) in v.iter_mut().enumerate() {
                let i = (bits >> (p * 3) & 7) as u16;
                *slot = match (i, a0 > a1) {
                    (0, _) => a0,
                    (1, _) => a1,
                    (i, true) => ((8 - i) * a0 + (i - 1) * a1) / 7,
                    (6, false) => 0,
                    (7, false) => 255,
                    (i, false) => ((6 - i) * a0 + (i - 1) * a1) / 5,
                } as u8;
            }
        }
    }
    Some(v)
}
