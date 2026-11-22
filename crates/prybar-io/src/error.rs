use thiserror::Error;

/// Errors raised while decoding untrusted binary input.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {
    #[error("read of {len} bytes at offset {offset} exceeds buffer of {size} bytes")]
    OutOfBounds {
        offset: usize,
        len: usize,
        size: usize,
    },
    #[error("count {count} exceeds sanity cap {cap}")]
    CountTooLarge { count: usize, cap: usize },
    #[error("string at offset {offset} has no terminator within {max} bytes")]
    Unterminated { offset: usize, max: usize },
    #[error("string at offset {offset} is not valid UTF-8")]
    InvalidUtf8 { offset: usize },
}

pub type Result<T> = std::result::Result<T, Error>;
