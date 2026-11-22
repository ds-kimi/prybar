use std::path::PathBuf;

/// Failures of the config store and the tool wrappers.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("io error on {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("invalid config {path}: {message}")]
    Config { path: PathBuf, message: String },
    #[error("no user config directory on this system")]
    NoConfigDir,
    #[error("unknown game `{0}`")]
    UnknownGame(String),
    #[error("game `{game}` has no {tool} path configured")]
    MissingTool { game: String, tool: &'static str },
    #[error("cannot start {exe}: {source}")]
    Spawn {
        exe: PathBuf,
        source: std::io::Error,
    },
}

impl ToolError {
    pub(crate) fn io(path: &std::path::Path, source: std::io::Error) -> Self {
        Self::Io {
            path: path.to_path_buf(),
            source,
        }
    }

    pub(crate) fn config(path: &std::path::Path, message: &str) -> Self {
        Self::Config {
            path: path.to_path_buf(),
            message: message.to_string(),
        }
    }
}
