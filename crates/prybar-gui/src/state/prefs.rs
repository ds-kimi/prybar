//! Choices remembered between runs: for now, only the theme.

use std::path::{Path, PathBuf};

use gpui::Context;

use super::{AppState, ToastKind};

/// `gui.txt` next to the games config, so `PRYBAR_CONFIG` moves both.
pub fn default_path() -> Option<PathBuf> {
    let config = prybar_tools::config::config_path().ok()?;
    Some(config.parent()?.join("gui.txt"))
}

/// A missing, unreadable or foreign file means the default dark theme.
pub fn load_light(path: &Path) -> bool {
    std::fs::read_to_string(path)
        .map(|text| text.lines().any(|l| l.trim() == "theme=light"))
        .unwrap_or(false)
}

pub fn save_light(path: &Path, light: bool) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let theme = if light { "light" } else { "dark" };
    std::fs::write(path, format!("theme={theme}\n"))
}

impl AppState {
    /// Flips the theme and writes it down when a prefs file is set;
    /// tests leave it unset so they never touch the real config.
    pub fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        self.light = !self.light;
        crate::app::follow_appearance(self.light, cx);
        if let Some(path) = self.prefs.clone() {
            if let Err(e) = save_light(&path, self.light) {
                self.toast(ToastKind::Error, format!("Theme not saved: {e}"), cx);
            }
        }
        cx.notify();
    }
}
