//! Compressed animation stream -> local bone poses per frame.

mod bone;
mod chain;
mod data;
mod frames;
mod section;
mod value;

pub use data::AnimData;
pub use frames::{frames, rest_frames};
pub use section::Layout;
