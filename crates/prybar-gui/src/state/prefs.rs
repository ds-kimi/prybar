//! Choices remembered between runs: theme, auto-update, analytics.

use std::path::{Path, PathBuf};

use gpui::Context;

use super::{AppState, ToastKind};

/// `gui.txt` next to the games config, so `PRYBAR_CONFIG` moves both.
pub fn default_path() -> Option<PathBuf> {
    let config = prybar_tools::config::config_path().ok()?;
    Some(config.parent()?.join("gui.txt"))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Prefs {
    pub light: bool,
    pub auto_update: bool,
    pub telemetry: bool,
    pub notice_seen: bool,
}

/// A missing, unreadable or foreign file means the defaults: dark theme,
/// auto-update and analytics on, notice not seen yet. Only an explicit
/// line turns any away from them.
pub fn load(path: &Path) -> Prefs {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let has = |line: &str| text.lines().any(|l| l.trim() == line);
    Prefs {
        light: has("theme=light"),
        auto_update: !has("auto_update=off"),
        telemetry: !has("telemetry=off"),
        notice_seen: has("telemetry_notice=seen"),
    }
}

impl AppState {
    /// Writes every choice down when a prefs file is set; tests leave it
    /// unset so they never touch the real config.
    pub fn save_prefs(&mut self, cx: &mut Context<Self>) {
        let Some(path) = self.prefs.clone() else {
            return;
        };
        let theme = if self.light { "light" } else { "dark" };
        let auto = if self.updates.auto { "on" } else { "off" };
        let tele = if self.privacy.on { "on" } else { "off" };
        let seen = if self.privacy.notice_seen {
            "seen"
        } else {
            "new"
        };
        let text = format!(
            "theme={theme}\nauto_update={auto}\ntelemetry={tele}\ntelemetry_notice={seen}\n"
        );
        let written = path
            .parent()
            .map_or(Ok(()), std::fs::create_dir_all)
            .and_then(|()| std::fs::write(&path, text));
        if let Err(e) = written {
            self.toast(ToastKind::Error, format!("Settings not saved: {e}"), cx);
        }
    }

    pub fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        self.light = !self.light;
        crate::app::follow_appearance(self.light, cx);
        self.save_prefs(cx);
        cx.notify();
    }
}
