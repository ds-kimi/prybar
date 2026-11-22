//! Garry's Mod addon archives (GMA v3): read, extract and pack with the
//! game's file whitelist enforced.

mod error;
mod extract;
mod format;
mod gma;
mod pack;

pub use error::{Error, Result};
pub use extract::{extract_all, extract_one};
pub use format::{Entry, Header};
pub use gma::Gma;
pub use pack::{is_whitelisted, pack, AddonJson};
