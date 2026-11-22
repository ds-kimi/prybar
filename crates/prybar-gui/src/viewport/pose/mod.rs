//! Sequence playback on the CPU: bone matrices per frame from the IR,
//! then the bind vertices skinned by them. Preview models are small, so
//! the GPU path stays the same as for a still model.

mod clip;
#[cfg(test)]
mod fixtures;
mod play;
mod skin;
#[cfg(test)]
mod tests;
mod world;

pub use clip::Clip;
pub use play::{Play, SPEEDS};
pub use skin::{joints, skin};
