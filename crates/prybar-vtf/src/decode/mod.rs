//! Pixel decoding into straight 8-bit RGBA.

mod dxt;
mod plain;

use crate::format::Format;

/// Decodes `data` (exactly one image of `w` x `h`) into RGBA bytes.
/// The caller has checked that `data` holds `format.size(w, h)` bytes.
pub fn rgba(format: Format, data: &[u8], w: u32, h: u32) -> Vec<u8> {
    let (w, h) = (w as usize, h as usize);
    let mut out = vec![0u8; w * h * 4];
    match format {
        Format::Plain { bpp, order } => plain::bytes(data, bpp, order, &mut out),
        Format::Rgb565 { bgr } => plain::rgb565(data, bgr, &mut out),
        Format::Dxt1 => dxt::blocks(data, w, h, 8, &mut out, dxt::Alpha::None),
        Format::Dxt3 => dxt::blocks(data, w, h, 16, &mut out, dxt::Alpha::Explicit),
        Format::Dxt5 => dxt::blocks(data, w, h, 16, &mut out, dxt::Alpha::Interpolated),
    }
    out
}
