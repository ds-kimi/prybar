//! The only payloads that ever leave the machine.

use serde::{Deserialize, Serialize};

/// Longest crash message kept after sanitizing.
pub const MAX_MESSAGE: usize = 500;
/// Most backtrace frames kept, all from prybar's own code.
pub const MAX_FRAMES: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case", deny_unknown_fields)]
pub enum Event {
    AppStart {
        version: String,
        os: String,
        arch: String,
    },
    Crash {
        version: String,
        os: String,
        arch: String,
        message: String,
        frames: Vec<String>,
    },
}

impl Event {
    pub fn app_start(version: &str) -> Self {
        Self::AppStart {
            version: version.into(),
            os: std::env::consts::OS.into(),
            arch: std::env::consts::ARCH.into(),
        }
    }

    /// Callers pass already sanitized text; see `report`.
    pub fn crash(version: &str, message: String, frames: Vec<String>) -> Self {
        Self::Crash {
            version: version.into(),
            os: std::env::consts::OS.into(),
            arch: std::env::consts::ARCH.into(),
            message,
            frames,
        }
    }

    /// Shape checks the collector runs before storing anything, so a
    /// forged request cannot smuggle free text past the limits.
    pub fn valid(&self) -> bool {
        let short =
            |s: &str| !s.is_empty() && s.len() <= 32 && s.bytes().all(|b| b.is_ascii_graphic());
        match self {
            Self::AppStart { version, os, arch } => short(version) && short(os) && short(arch),
            Self::Crash {
                version,
                os,
                arch,
                message,
                frames,
            } => {
                short(version)
                    && short(os)
                    && short(arch)
                    && message.len() <= MAX_MESSAGE
                    && frames.len() <= MAX_FRAMES
                    && frames
                        .iter()
                        .all(|f| f.len() <= 200 && f.starts_with("prybar"))
            }
        }
    }
}
