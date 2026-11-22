use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] prybar_io::Error),
    #[error("not an MDL file (magic {0:?})")]
    BadMagic([u8; 4]),
    #[error("MDL version {version} is not supported yet")]
    Unsupported { version: i32 },
    #[error("{what} has invalid value {value}")]
    Invalid { what: &'static str, value: i64 },
}

pub type Result<T> = std::result::Result<T, Error>;
