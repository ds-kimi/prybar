//! Workshop form rules, from the limits Steam puts on an item.

/// Steam cuts titles past this.
const TITLE_MAX: usize = 128;
/// Steam's description limit.
const DESCRIPTION_MAX: usize = 8000;

pub fn title(text: &str) -> Option<&'static str> {
    let n = text.trim().chars().count();
    if n == 0 {
        return Some("Give the addon a title.");
    }
    (n > TITLE_MAX).then_some("Keep the title to 128 characters or fewer.")
}

pub fn description(text: &str) -> Option<&'static str> {
    (text.chars().count() > DESCRIPTION_MAX)
        .then_some("Keep the description to 8000 characters or fewer.")
}

pub fn tags(text: &str) -> Option<&'static str> {
    let empty = text.split(',').any(|t| t.trim().is_empty());
    (!text.trim().is_empty() && empty)
        .then_some("Separate tags with single commas, like model, prop.")
}
