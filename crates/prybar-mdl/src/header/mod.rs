//! `studiohdr_t` for v44-v53 (plus the `studiohdr2_t` long-name copy).

pub mod fields;
mod hdr2;
mod parse;

use crate::ir::Vec3;

/// Offset of the first plain int field after the fixed vectors.
const INTS_START: usize = 152;
/// `studiohdr_t` is 408 bytes in every version from 44 to 52.
pub const SIZE: usize = 408;

/// Fixed fields are named; the long run of `(count, index)` pairs is kept
/// as raw ints and read through [`fields`] so one struct serves 44-53.
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub version: i32,
    pub checksum: i32,
    pub name: String,
    pub length: i32,
    /// eye, illum, hull min/max, view min/max, in file order.
    pub vectors: [Vec3; 6],
    ints: Vec<i32>,
}

impl Header {
    /// Reads the int at a `fields` offset; offsets outside the header, or
    /// fields the version dropped, give 0.
    pub fn int(&self, field: usize) -> i32 {
        if self.version == 53 && field > fields::INCLUDE_MODELS + 4 {
            return 0;
        }
        let idx = field.wrapping_sub(INTS_START) / 4;
        self.ints.get(idx).copied().unwrap_or(0)
    }

    /// `(count, index)` pair starting at the given count field.
    pub fn span(&self, count_field: usize) -> (i32, i32) {
        (self.int(count_field), self.int(count_field + 4))
    }

    pub fn float(&self, field: usize) -> f32 {
        f32::from_bits(self.int(field) as u32)
    }
}
