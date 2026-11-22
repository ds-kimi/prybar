//! WGSL sources, embedded so the binary needs no files at run time.

pub const MODEL: &str = include_str!("model.wgsl");
pub const LINES: &str = include_str!("lines.wgsl");
