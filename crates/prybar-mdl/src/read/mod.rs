//! Bounds-checked helpers shared by every MDL version parser.

mod prim;
mod source;
mod table;
mod words;

pub use prim::{mat3x4, quat, vec3};
pub use source::Source;
pub use table::{name_at, offset, table};
pub use words::Words;
