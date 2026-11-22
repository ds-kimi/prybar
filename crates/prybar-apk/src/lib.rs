//! APK packages (id 0x2357): read and extract only.
//!
//! Layout: a 16-byte header (id, files offset, file count, directory
//! offset), then a directory of (name size, NUL-terminated name, data
//! offset, data size, next entry offset, reserved) records. Data is stored
//! uncompressed with no checksum.

mod apk;
mod error;
mod extract;

pub use apk::{Apk, Entry, APK_ID};
pub use error::{Error, Result};
pub use extract::{extract_all, extract_one};
