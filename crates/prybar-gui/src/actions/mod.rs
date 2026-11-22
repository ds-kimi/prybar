//! Actions and key bindings shared by every design.

use gpui::{actions, KeyBinding};

actions!(
    prybar,
    [
        Open,
        Back,
        Search,
        ShowChooser,
        ShowWorkbench,
        ShowCanvas,
        ShowConsole,
        TogglePalette,
        ToggleTheme,
        FocusNext,
        FocusPrev,
        Submit,
        SelectNext,
        SelectPrev,
        Quit
    ]
);

/// The command key of the host: cmd on macOS, ctrl elsewhere.
pub const MOD: &str = if cfg!(target_os = "macos") {
    "cmd"
} else {
    "ctrl"
};

/// Key context the root sets while the console design is shown.
pub const CONSOLE: &str = "Console";

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

/// Bound without a context so they work wherever focus is, except the
/// list keys, which only the console's command bar uses.
pub fn bindings() -> Vec<KeyBinding> {
    vec![
        KeyBinding::new(key!("o"), Open, None),
        KeyBinding::new(key!("["), Back, None),
        KeyBinding::new("escape", Back, None),
        KeyBinding::new(key!("f"), Search, None),
        KeyBinding::new(key!("0"), ShowChooser, None),
        KeyBinding::new(key!("1"), ShowWorkbench, None),
        KeyBinding::new(key!("2"), ShowCanvas, None),
        KeyBinding::new(key!("3"), ShowConsole, None),
        KeyBinding::new(key!("k"), TogglePalette, None),
        KeyBinding::new(key!("shift-p"), TogglePalette, None),
        KeyBinding::new(key!("shift-l"), ToggleTheme, None),
        KeyBinding::new("tab", FocusNext, None),
        KeyBinding::new("shift-tab", FocusPrev, None),
        KeyBinding::new(key!("q"), Quit, None),
        KeyBinding::new("enter", Submit, Some(CONSOLE)),
        KeyBinding::new("down", SelectNext, Some(CONSOLE)),
        KeyBinding::new("up", SelectPrev, Some(CONSOLE)),
    ]
}
