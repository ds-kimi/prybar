use super::key::decode_name;
use crate::{Error, Result};
use prybar_io::Reader;

pub const CENTRAL_SIG: u32 = 0x0201_4648;
pub const CENTRAL_LEN: usize = 46;
const LOCAL_LEN: usize = 30;

/// One central-directory record, with the fields prybar needs.
#[derive(Debug, Clone)]
pub struct Central {
    pub name: String,
    pub stored_size: u32,
    pub size: u32,
    pub local: usize,
    /// Offset of the next central record.
    pub next: usize,
}

pub fn central(r: &Reader, at: usize) -> Result<Central> {
    expect_sig(r, at, CENTRAL_SIG)?;
    let name_len = r.read::<u16>(at + 28)? as usize;
    let extra = r.read::<u16>(at + 30)? as usize + r.read::<u16>(at + 32)? as usize;
    let name_at = at + CENTRAL_LEN;
    Ok(Central {
        name: decode_name(r.bytes(name_at, name_len)?, name_at),
        stored_size: r.read::<u32>(at + 20)?,
        size: r.read::<u32>(at + 24)?,
        local: r.read::<u32>(at + 42)? as usize,
        next: name_at + name_len + extra,
    })
}

/// Local headers carry their own name and extra lengths, which may differ
/// from the central copy, so the data offset must come from here. Their
/// signature is not checked: real packages are not consistent about it.
pub fn data_offset(r: &Reader, local: usize) -> Result<usize> {
    let name_len = r.read::<u16>(local + 26)? as usize;
    let extra = r.read::<u16>(local + 28)? as usize;
    Ok(local.saturating_add(LOCAL_LEN + name_len + extra))
}

fn expect_sig(r: &Reader, at: usize, sig: u32) -> Result<()> {
    let found = r.read::<u32>(at)?;
    if found != sig {
        return Err(Error::BadSignature { found, offset: at });
    }
    Ok(())
}
