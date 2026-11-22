use thiserror::Error;

/// Everything that can stop a download or an install.
#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("network: {0}")]
    Network(String),
    #[error("release data is invalid: {0}")]
    BadRelease(String),
    #[error("no release asset for {0}")]
    MissingAsset(String),
    #[error("release is not signed; download it from {0}")]
    Unsigned(String),
    #[error("signature of SHA256SUMS does not match the embedded key")]
    BadSignature,
    #[error("checksum mismatch for {0}")]
    BadChecksum(String),
    #[error("download is larger than {0} bytes")]
    TooLarge(u64),
    #[error("archive is invalid: {0}")]
    BadArchive(String),
    #[error("cannot install here: {0}")]
    NotInstallable(String),
    #[error("i/o: {0}")]
    Io(#[from] std::io::Error),
}
