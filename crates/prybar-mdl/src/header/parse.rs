use super::{hdr2, Header, INTS_START, SIZE};
use crate::read::{vec3, Source};
use crate::Result;
use prybar_io::Cursor;

/// v53 inserts `sznameindex` after the checksum, which pushes every
/// later field 4 bytes down, and ends with the embedded-file offsets.
const V53_SHIFT: usize = 4;
const V53_SIZE: usize = 476;

impl Header {
    pub fn parse(r: &Source) -> Result<Self> {
        let version: i32 = r.read(4)?;
        let shift = if version == 53 { V53_SHIFT } else { 0 };
        r.bytes(0, if version == 53 { V53_SIZE } else { SIZE })?;
        let mut c = Cursor::new(r.reader, 12 + shift);
        let raw = c.bytes(64)?;
        let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
        let length = c.read()?;
        let mut vectors = [[0.0; 3]; 6];
        for v in &mut vectors {
            *v = vec3(&mut c)?;
        }
        let ints = r.array(INTS_START + shift, (SIZE - INTS_START) / 4, 64)?;
        let mut hdr = Self {
            version,
            checksum: r.read(8)?,
            name: String::from_utf8_lossy(&raw[..end]).into_owned(),
            length,
            vectors,
            ints,
        };
        hdr2::apply_long_name(r, &mut hdr)?;
        Ok(hdr)
    }
}
