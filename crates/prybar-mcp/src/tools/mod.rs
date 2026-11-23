//! One `#[tool_router]` block per area, merged in `server`: a single
//! block would hold every tool in one file.

mod check;
mod config;
mod external;
mod model;
mod package;
mod update;
mod workshop;

pub use update::{background_check, update_notice};
