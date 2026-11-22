//! One `#[tool_router]` block per area, merged in `server`: a single
//! block would hold every tool in one file.

mod config;
mod external;
mod model;
mod package;
mod update;

pub use update::{background_check, update_notice};
