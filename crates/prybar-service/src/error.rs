use std::fmt::Display;
use std::path::{Path, PathBuf};

/// Process exit codes. They are part of the public interface: scripts and
/// agents branch on them, so values never change once released.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExitCode {
    Ok = 0,
    Internal = 1,
    Usage = 2,
    NotFound = 3,
    InvalidInput = 4,
    Io = 5,
    Tool = 6,
    Partial = 7,
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("{0}")]
    Usage(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("{input}: {message}")]
    Invalid { input: String, message: String },
    #[error("io error on {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{0}")]
    Tool(String),
}

impl ServiceError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::Usage(_) => ExitCode::Usage,
            Self::NotFound(_) => ExitCode::NotFound,
            Self::Invalid { .. } => ExitCode::InvalidInput,
            Self::Io { .. } => ExitCode::Io,
            Self::Tool(_) => ExitCode::Tool,
        }
    }

    pub fn invalid(input: impl Display, err: impl Display) -> Self {
        Self::Invalid {
            input: input.to_string(),
            message: err.to_string(),
        }
    }

    pub fn io(path: &Path, source: std::io::Error) -> Self {
        Self::Io {
            path: path.to_path_buf(),
            source,
        }
    }
}
