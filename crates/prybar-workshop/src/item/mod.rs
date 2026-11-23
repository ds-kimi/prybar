//! Editing an item the user already published, plus what the app keeps
//! about it locally: the linked addon folder and stats over time.

mod edit;
mod links;
mod snapshot;

pub use edit::{changes, Draft};
pub use links::Links;
pub use snapshot::{record, series, Point};
