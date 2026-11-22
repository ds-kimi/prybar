//! MDL v53 (Titanfall 2): the v44-v49 tables with a shifted header, a
//! reordered bone struct, a slimmer animdesc and the VVD, VTX and PHY
//! files embedded in the MDL itself.

mod anims;
mod bones;
mod embedded;

pub use anims::animations;
pub use bones::bones;
pub use embedded::{embedded, Embedded};
