use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Format(#[from] prybar_io::Error),
    #[error("not a VPK directory file")]
    BadSignature,
    #[error("unsupported VPK version {0}")]
    BadVersion(u32),
    #[error("bad entry terminator at offset {0}")]
    BadTerminator(usize),
    #[error("CRC mismatch for {0}")]
    Crc(String),
    #[error("unsafe entry path {0:?}")]
    UnsafePath(String),
    #[error("multi-archive output name must end with _dir.vpk")]
    BadOutputName,
    #[error("too many archives or data too large for VPK")]
    TooLarge,
    #[error("path is not valid UTF-8: {0:?}")]
    NonUtf8(PathBuf),
    #[error(transparent)]
    Walk(#[from] walkdir::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
