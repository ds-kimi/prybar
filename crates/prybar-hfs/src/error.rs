use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Format(#[from] prybar_io::Error),
    #[error("not an HFS package: no end-of-directory record")]
    NoDirectory,
    #[error("bad record signature {found:#010x} at offset {offset}")]
    BadSignature { found: u32, offset: usize },
    #[error("unsafe entry path {0:?}")]
    UnsafePath(String),
    #[error("{0:?} is a .comp entry; its compression is not supported")]
    Unsupported(String),
}

pub type Result<T> = std::result::Result<T, Error>;
