use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum WorkshopError {
    #[error("invalid {field}: {reason}")]
    Invalid { field: &'static str, reason: String },
    #[error("io error on {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("built without the `workshop` feature; rebuild with --features workshop")]
    Disabled,
    #[error("steam: {0}")]
    Steam(String),
}

impl WorkshopError {
    pub(crate) fn invalid(field: &'static str, reason: impl Into<String>) -> Self {
        Self::Invalid {
            field,
            reason: reason.into(),
        }
    }
}
