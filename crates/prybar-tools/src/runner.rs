//! How a Windows tool binary is started on this OS.

use std::path::{Path, PathBuf};
use std::process::Command;

/// studiomdl and hlmv only exist as Windows binaries; elsewhere they go
/// through Wine (or a Proton-style wrapper with the same CLI).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Runner {
    Native,
    Wine(PathBuf),
}

impl Runner {
    /// Native on Windows; elsewhere `PRYBAR_WINE`, else `wine` on PATH.
    pub fn for_host() -> Self {
        if cfg!(windows) {
            return Runner::Native;
        }
        let wine = std::env::var_os("PRYBAR_WINE").unwrap_or_else(|| "wine".into());
        Runner::Wine(PathBuf::from(wine))
    }

    /// `exe` with no arguments yet, ready for the caller to extend.
    pub fn command(&self, exe: &Path) -> Command {
        match self {
            Runner::Native => Command::new(exe),
            Runner::Wine(wine) => {
                let mut cmd = Command::new(wine);
                cmd.arg(exe);
                cmd
            }
        }
    }
}
