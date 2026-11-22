//! Actions and key bindings of the app.

use gpui::{actions, KeyBinding};

actions!(
    prybar,
    [
        Open,
        Back,
        Search,
        TogglePalette,
        ToggleTheme,
        FocusNext,
        FocusPrev,
        Quit
    ]
);

/// The command key of the host: cmd on macOS, ctrl elsewhere.
pub const MOD: &str = if cfg!(target_os = "macos") {
    "cmd"
} else {
    "ctrl"
};

/// `MOD-<key>` as a static string, which key bindings need.
macro_rules! key {
    ($k:literal) => {
        if cfg!(target_os = "macos") {
            concat!("cmd-", $k)
        } else {
            concat!("ctrl-", $k)
        }
    };
}

/// Bound without a context so they work wherever focus is.
pub fn bindings() -> Vec<KeyBinding> {
    vec![
        KeyBinding::new(key!("o"), Open, None),
        KeyBinding::new(key!("["), Back, None),
        KeyBinding::new("escape", Back, None),
        KeyBinding::new(key!("f"), Search, None),
        KeyBinding::new(key!("k"), TogglePalette, None),
        KeyBinding::new(key!("shift-p"), TogglePalette, None),
        KeyBinding::new(key!("shift-l"), ToggleTheme, None),
        KeyBinding::new("tab", FocusNext, None),
        KeyBinding::new("shift-tab", FocusPrev, None),
        KeyBinding::new(key!("q"), Quit, None),
    ]
}
