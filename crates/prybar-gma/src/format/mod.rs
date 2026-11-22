//! On-disk layout: header, file table, encoder.

mod header;
mod table;
mod write;

pub(crate) use header::parse_header;
pub use header::Header;
pub(crate) use table::parse_table;
pub use table::Entry;
pub(crate) use write::encode;

pub(crate) const MAGIC: &[u8; 4] = b"GMAD";
pub(crate) const VERSION: u8 = 3;
