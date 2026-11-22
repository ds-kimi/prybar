//! Pixel formats and their storage sizes.

/// Formats the decoder understands. Other ids (floats, palettes) are
/// rejected with `Error::Format` instead of guessed at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// Byte-per-channel layout: `order[i]` is the byte index of R, G, B, A,
    /// or `None` when the channel is absent (filled with 255, or grey).
    Plain {
        bpp: usize,
        order: [Option<u8>; 4],
    },
    /// 16-bit packed 5-6-5; `bgr` swaps red and blue.
    Rgb565 {
        bgr: bool,
    },
    Dxt1,
    Dxt3,
    Dxt5,
}

const fn plain(bpp: usize, order: [Option<u8>; 4]) -> Format {
    Format::Plain { bpp, order }
}

impl Format {
    /// Maps the `IMAGE_FORMAT_*` id stored in the header.
    pub fn from_id(id: i32) -> Option<Format> {
        let (n, s) = (None, Some);
        Some(match id {
            0 => plain(4, [s(0), s(1), s(2), s(3)]),
            1 => plain(4, [s(3), s(2), s(1), s(0)]),
            2 | 9 => plain(3, [s(0), s(1), s(2), n]),
            3 | 10 => plain(3, [s(2), s(1), s(0), n]),
            4 => Format::Rgb565 { bgr: false },
            // I8 and IA88 are grey: every color channel reads byte 0.
            5 => plain(1, [s(0), s(0), s(0), n]),
            6 => plain(2, [s(0), s(0), s(0), s(1)]),
            8 => plain(1, [n, n, n, s(0)]),
            11 => plain(4, [s(1), s(2), s(3), s(0)]),
            12 => plain(4, [s(2), s(1), s(0), s(3)]),
            13 | 20 => Format::Dxt1,
            14 => Format::Dxt3,
            15 => Format::Dxt5,
            16 => plain(4, [s(2), s(1), s(0), n]),
            17 => Format::Rgb565 { bgr: true },
            22 => plain(2, [s(0), s(1), n, n]),
            23 | 26 => plain(4, [s(0), s(1), s(2), s(3)]),
            _ => return None,
        })
    }

    /// Bytes one `w` x `h` image takes, or `None` on overflow.
    pub fn size(self, w: u32, h: u32) -> Option<usize> {
        let (w, h) = (w.max(1) as usize, h.max(1) as usize);
        match self {
            Format::Plain { bpp, .. } => w.checked_mul(h)?.checked_mul(bpp),
            Format::Rgb565 { .. } => w.checked_mul(h)?.checked_mul(2),
            Format::Dxt1 => w.div_ceil(4).checked_mul(h.div_ceil(4))?.checked_mul(8),
            Format::Dxt3 | Format::Dxt5 => {
                w.div_ceil(4).checked_mul(h.div_ceil(4))?.checked_mul(16)
            }
        }
    }
}

const NAMES: [&str; 27] = [
    "RGBA8888",
    "ABGR8888",
    "RGB888",
    "BGR888",
    "RGB565",
    "I8",
    "IA88",
    "P8",
    "A8",
    "RGB888 bluescreen",
    "BGR888 bluescreen",
    "ARGB8888",
    "BGRA8888",
    "DXT1",
    "DXT3",
    "DXT5",
    "BGRX8888",
    "BGR565",
    "BGRX5551",
    "BGRA4444",
    "DXT1 1-bit alpha",
    "BGRA5551",
    "UV88",
    "UVWQ8888",
    "RGBA16161616F",
    "RGBA16161616",
    "UVLX8888",
];

/// Display name of an `IMAGE_FORMAT_*` id.
pub fn name(id: i32) -> &'static str {
    usize::try_from(id)
        .ok()
        .and_then(|i| NAMES.get(i))
        .copied()
        .unwrap_or("unknown")
}
