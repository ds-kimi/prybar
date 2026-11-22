#[cfg(not(feature = "workshop"))]
mod off;
#[cfg(feature = "workshop")]
mod steam;

#[cfg(not(feature = "workshop"))]
pub use off::{list, publish};
#[cfg(feature = "workshop")]
pub use steam::{list, publish};
