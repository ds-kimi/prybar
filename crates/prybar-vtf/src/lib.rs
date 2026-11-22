//! Reads Source engine VTF textures (7.0 to 7.5) into RGBA images.
//!
//! Every size and offset comes from the file, so all of them are checked
//! against the buffer before use; a hostile file yields an `Error`.

mod decode;
mod error;
mod format;
mod header;
mod layout;
mod read;

pub use error::{Error, Result};
pub use format::Format;
pub use header::{Header, MAX_SIDE};

/// One decoded image, 8-bit straight RGBA, rows top to bottom.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// A parsed texture that borrows nothing: decoding reads `bytes` on demand.
#[derive(Debug, Clone)]
pub struct Vtf {
    pub header: Header,
    bytes: Vec<u8>,
}

impl Vtf {
    pub fn parse(bytes: Vec<u8>) -> Result<Vtf> {
        let header = Header::parse(&bytes)?;
        Ok(Vtf { header, bytes })
    }

    /// Decodes mip `level` (0 = full size) of the first frame and face.
    pub fn decode(&self, level: u8) -> Result<Image> {
        let (range, width, height) = layout::mip_range(&self.header, level)?;
        let data = self.bytes.get(range).ok_or(Error::Truncated)?;
        let rgba = decode::rgba(self.header.format, data, width, height);
        Ok(Image {
            width,
            height,
            rgba,
        })
    }

    /// Human name of the stored pixel format, for display.
    pub fn format_name(&self) -> &'static str {
        format::name(self.header.format_id)
    }
}
