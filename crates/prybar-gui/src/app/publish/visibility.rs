//! The Workshop visibility choice.

use gpui::{App, Entity};

use crate::designs::select::{Choice, Select};

/// The words the spec parser reads, in the order of [`visibility`]'s choices.
pub(super) const VALUES: [&str; 4] = ["public", "friends", "private", "unlisted"];

/// Steam's four visibilities in its own words; new items start hidden
/// so nothing goes public by accident.
pub fn visibility(cx: &mut App) -> Entity<Select> {
    let choice = |label: &'static str, tip: &'static str| Choice {
        label: label.into(),
        tip: tip.into(),
    };
    let choices = vec![
        choice("Public", "Everyone can find it and subscribe"),
        choice("Friends only", "Only your Steam friends can see it"),
        choice("Hidden", "Only you can see it, until you change this"),
        choice(
            "Unlisted",
            "Not in search, but anyone with the link can open it",
        ),
    ];
    Select::new("pb-pub-visibility", choices, 2, cx)
}
