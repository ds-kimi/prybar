//! Entry for crates that reuse these builders through `#[path]`.
//! A path-attribute file resolves its children next to itself, so this works
//! on every OS (an inline `mod` would need a directory that does not exist).

pub mod builder;
pub mod mesh_records;
pub mod records;
