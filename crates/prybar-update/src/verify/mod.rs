mod sums;

pub use sums::parse_sums;

use crate::UpdateError;
use minisign_verify::{PublicKey, Signature};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;

/// Checks the minisign signature of `sums` with `public_key` (the `.pub`
/// file text). Legacy non-prehashed signatures are refused.
pub fn verify_signature(sums: &[u8], signature: &str, public_key: &str) -> Result<(), UpdateError> {
    let key = PublicKey::decode(public_key)
        .or_else(|_| PublicKey::from_base64(public_key.trim()))
        .map_err(|_| UpdateError::BadSignature)?;
    let sig = Signature::decode(signature).map_err(|_| UpdateError::BadSignature)?;
    key.verify(sums, &sig, false)
        .map_err(|_| UpdateError::BadSignature)
}

/// SHA-256 of a file, streamed so large archives stay out of memory.
pub fn sha256_file(path: &Path) -> Result<[u8; 32], UpdateError> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            return Ok(hasher.finalize().into());
        }
        hasher.update(&buf[..n]);
    }
}
