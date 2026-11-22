use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Format(#[from] prybar_io::Error),
    #[error("not an APK package")]
    BadMagic,
    #[error("unsafe entry path {0:?}")]
    UnsafePath(String),
}

pub type Result<T> = std::result::Result<T, Error>;
