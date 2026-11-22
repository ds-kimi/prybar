use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("not a VTF file")]
    BadMagic,
    #[error("unsupported VTF version 7.{0}")]
    BadVersion(u32),
    #[error("VTF file is truncated")]
    Truncated,
    #[error("unsupported VTF image format {0}")]
    Format(i32),
    #[error("VTF size {0}x{1} is out of range")]
    Size(u32, u32),
    #[error("mip level {0} does not exist")]
    Mip(u8),
}

pub type Result<T> = std::result::Result<T, Error>;
