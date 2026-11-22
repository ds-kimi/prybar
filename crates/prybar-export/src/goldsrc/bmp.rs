use prybar_mdl::ir::Texture;

const HEADERS: usize = 14 + 40 + 256 * 4;

/// 8-bit paletted BMP, rows bottom-up and padded to 4 bytes. `None` when
/// the pixel or palette data does not match the declared size.
pub fn bmp(t: &Texture) -> Option<Vec<u8>> {
    let (w, h) = (t.width as usize, t.height as usize);
    if w == 0 || h == 0 || t.pixels.len() != w * h || t.palette.len() != 768 {
        return None;
    }
    let stride = (w + 3) & !3;
    let size = HEADERS + stride * h;
    let mut out = Vec::with_capacity(size);
    header(&mut out, size, t.width, t.height);
    for rgb in t.palette.as_chunks::<3>().0 {
        out.extend_from_slice(&[rgb[2], rgb[1], rgb[0], 0]);
    }
    for row in t.pixels.chunks_exact(w).rev() {
        out.extend_from_slice(row);
        out.resize(out.len() + stride - w, 0);
    }
    Some(out)
}

fn header(out: &mut Vec<u8>, size: usize, width: u32, height: u32) {
    let le = |out: &mut Vec<u8>, v: u32| out.extend_from_slice(&v.to_le_bytes());
    out.extend_from_slice(b"BM");
    le(out, size as u32);
    le(out, 0);
    le(out, HEADERS as u32);
    le(out, 40);
    le(out, width);
    le(out, height);
    // One plane, 8 bits per pixel, then no compression.
    le(out, 1 | (8 << 16));
    le(out, 0);
    le(out, (size - HEADERS) as u32);
    le(out, 2835);
    le(out, 2835);
    le(out, 256);
    le(out, 256);
}
