//! Eyeballs and the eyelid flexes bound to them.

mod eyeball;
mod eyelid;
mod name;

pub use eyeball::write as eyeballs;
pub use eyelid::{is_lid, write as eyelids};
