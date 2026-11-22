//! Bounds-checked little-endian binary reading and writing.
//!
//! Every input file prybar parses is untrusted, so all reads go through
//! offset-based accessors that return errors instead of panicking.

mod error;
mod path;
mod prim;
mod read;
mod write;

pub use error::{Error, Result};
pub use path::safe_join;
pub use prim::Primitive;
pub use read::Cursor;
pub use read::Reader;
pub use write::Writer;
