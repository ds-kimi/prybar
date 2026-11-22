//! HFS packages (Vindictus): read, list and extract only.
//!
//! HFS is a ZIP layout with its own signatures (`HF` instead of `PK`) and
//! entry names XOR-encoded with a fixed key indexed by file offset. Entries
//! named `*.comp` hold data in an undocumented compressed form: they are
//! listed but not extracted. See `docs/formats/hfs.md`.

mod error;
mod extract;
mod format;
mod hfs;

pub use error::{Error, Result};
pub use extract::{extract_all, extract_one};
pub use format::key::xor_name;
pub use hfs::{Entry, Hfs};
