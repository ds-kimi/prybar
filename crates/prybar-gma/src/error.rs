use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Format(#[from] prybar_io::Error),
    #[error("not a GMA file")]
    BadMagic,
    #[error("unsupported GMA version {0}")]
    BadVersion(u8),
    #[error("entry {0:?} has an invalid size")]
    BadSize(String),
    #[error("CRC mismatch for {0}")]
    Crc(String),
    #[error("unsafe entry path {0:?}")]
    UnsafePath(String),
    #[error("files not allowed by the gmod whitelist: {0:?}")]
    NotWhitelisted(Vec<String>),
    #[error("addon.json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("path is not valid UTF-8: {0:?}")]
    NonUtf8(PathBuf),
    #[error(transparent)]
    Walk(#[from] walkdir::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
