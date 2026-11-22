use super::layout::Layout;
use crate::ir::Vec3;
use crate::read::{vec3, Source};
use crate::Result;
use prybar_io::Cursor;

/// Ints read after the vectors; the longest legacy header has 52.
const INT_COUNT: usize = 52;

/// Legacy `studiohdr_t`: named fixed fields plus the raw int run that the
/// [`Layout`] slots index into.
pub struct LegacyHeader {
    pub version: i32,
    pub checksum: i32,
    pub name: String,
    /// eye, illum, hull min/max, view min/max, in file order.
    pub vectors: [Vec3; 6],
    ints: Vec<i32>,
}

impl LegacyHeader {
    pub fn parse(r: &Source, lay: &Layout) -> Result<Self> {
        let raw = r.bytes(12, lay.name_len)?;
        let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
        let vec_count = if lay.extra_vector { 7 } else { 6 };
        let mut c = Cursor::new(r.reader, 16 + lay.name_len);
        let mut all = Vec::with_capacity(vec_count);
        for _ in 0..vec_count {
            all.push(vec3(&mut c)?);
        }
        if lay.extra_vector {
            all.remove(2);
        }
        let ints_at = 16 + lay.name_len + vec_count * 12;
        Ok(Self {
            version: r.read(4)?,
            checksum: r.read(8)?,
            name: String::from_utf8_lossy(&raw[..end]).into_owned(),
            vectors: all.try_into().unwrap_or_default(),
            ints: r.array(ints_at, INT_COUNT, INT_COUNT)?,
        })
    }

    /// Int at a layout slot; slots past the run read 0.
    pub fn int(&self, slot: usize) -> i32 {
        self.ints.get(slot).copied().unwrap_or(0)
    }

    /// `(count, index)` pair starting at a slot.
    pub fn span(&self, slot: usize) -> (i32, i32) {
        (self.int(slot), self.int(slot + 1))
    }
}
