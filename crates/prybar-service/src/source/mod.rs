//! Where models come from: files on disk, folders, wildcards, or entries
//! inside a package (`pak_dir.vpk:models/foo.mdl`).

mod expand;
mod model_ref;
mod origin;
mod packed;
mod wildcard;

pub use expand::{expand, Batch};
pub use model_ref::ModelRef;
pub use origin::Origin;
pub use packed::mdl_stem;
pub use wildcard::matches;
