//! Valve Pak (VPK) v1/v2: read, extract and pack, single file or
//! multi-archive (`name_dir.vpk` + `name_NNN.vpk`).

mod entry;
mod error;
mod extract;
mod format;
mod pack;
mod vpk;

pub use entry::Entry;
pub use error::{Error, Result};
pub use extract::{extract_all, extract_one};
pub use format::{archive_path, Header};
pub use pack::{pack, PackOptions};
pub use vpk::Vpk;

/// Archive index meaning "data follows the tree inside the dir file".
pub const EMBEDDED: u16 = 0x7fff;
