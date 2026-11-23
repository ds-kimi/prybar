//! The desktop app's analytics choice and its local sent-data log, read
//! and changed from the CLI or MCP. Only the GUI ever sends anything; the
//! CLI and MCP server send nothing, so this is about the app.

use std::path::{Path, PathBuf};

use prybar_telemetry::{Entry, Store};
use serde::Serialize;

use crate::{Result, ServiceError};

#[derive(Debug, Clone, Serialize)]
pub struct PrivacyReport {
    /// The user's choice; on unless turned off.
    pub on: bool,
    /// Environment switch that blocks sending whatever `on` says.
    pub blocked_by: Option<&'static str>,
    pub prefs_file: PathBuf,
    pub log_file: PathBuf,
    /// Every payload the app queued or sent, oldest first.
    pub log: Vec<Entry>,
}

/// `set` writes the choice the way Settings › Privacy does (turning off
/// also drops anything still queued); `clear_log` empties the log.
pub fn privacy(set: Option<bool>, clear_log: bool) -> Result<PrivacyReport> {
    let prefs = prybar_tools::config::config_path()
        .ok()
        .and_then(|p| Some(p.parent()?.join("gui.txt")))
        .ok_or_else(|| ServiceError::NotFound("prybar config folder".into()))?;
    let store = Store::new(prefs.with_file_name("telemetry"));
    if let Some(on) = set {
        write_choice(&prefs, on)?;
        if !on {
            store
                .drop_queue()
                .map_err(|e| ServiceError::io(&store.log_path(), e))?;
        }
    }
    if clear_log {
        store
            .clear_log()
            .map_err(|e| ServiceError::io(&store.log_path(), e))?;
    }
    let text = std::fs::read_to_string(&prefs).unwrap_or_default();
    Ok(PrivacyReport {
        on: !text.lines().any(|l| l.trim() == "telemetry=off"),
        blocked_by: prybar_telemetry::blocked(),
        log_file: store.log_path(),
        log: store.entries(),
        prefs_file: prefs,
    })
}

/// Replaces only the `telemetry=` line so theme and list choices stay.
fn write_choice(prefs: &Path, on: bool) -> Result<()> {
    let text = std::fs::read_to_string(prefs).unwrap_or_default();
    let mut lines: Vec<&str> = text
        .lines()
        .filter(|l| !l.trim().starts_with("telemetry="))
        .collect();
    let line = if on { "telemetry=on" } else { "telemetry=off" };
    lines.push(line);
    let out = lines.join("\n") + "\n";
    if let Some(dir) = prefs.parent() {
        std::fs::create_dir_all(dir).map_err(|e| ServiceError::io(dir, e))?;
    }
    std::fs::write(prefs, out).map_err(|e| ServiceError::io(prefs, e))
}
