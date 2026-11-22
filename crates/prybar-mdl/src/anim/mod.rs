//! Animations: animdesc tables, compressed bone streams and the rotation
//! math shared by every Source version.

mod decode;
mod desc;
pub mod math;
mod packed;
mod root;
mod rotate;

pub use desc::animations;
pub use root::{movement_at, root_to_smd};
