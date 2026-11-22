//! Uncompressed formats.

/// Missing color channels read 0 and a missing alpha reads opaque.
pub fn bytes(data: &[u8], bpp: usize, order: [Option<u8>; 4], out: &mut [u8]) {
    for (px, dst) in data.chunks_exact(bpp).zip(out.as_chunks_mut::<4>().0) {
        for (c, slot) in order.iter().enumerate() {
            let fill = if c == 3 { 255 } else { 0 };
            dst[c] = slot
                .and_then(|i| px.get(i as usize).copied())
                .unwrap_or(fill);
        }
    }
}

pub fn rgb565(data: &[u8], bgr: bool, out: &mut [u8]) {
    for (px, dst) in data
        .as_chunks::<2>()
        .0
        .iter()
        .zip(out.as_chunks_mut::<4>().0)
    {
        let [r, g, b] = super::dxt::expand565(u16::from_le_bytes([px[0], px[1]]));
        let (r, b) = if bgr { (b, r) } else { (r, b) };
        dst.copy_from_slice(&[r, g, b, 255]);
    }
}
