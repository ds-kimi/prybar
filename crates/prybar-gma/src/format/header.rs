use super::{MAGIC, VERSION};
use crate::{Error, Result};
use prybar_io::Cursor;

/// Descriptions hold JSON and can be long; names and paths cannot.
pub(crate) const MAX_TEXT: usize = 1 << 20;
pub(crate) const MAX_NAME: usize = 4096;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Header {
    pub version: u8,
    pub steam_id: u64,
    pub timestamp: u64,
    pub required: Vec<String>,
    pub name: String,
    /// gmad stores a JSON object here (description, type, tags).
    pub description: String,
    pub author: String,
    pub addon_version: i32,
}

pub(crate) fn parse_header(c: &mut Cursor) -> Result<Header> {
    if c.bytes(4)? != MAGIC {
        return Err(Error::BadMagic);
    }
    let version = c.read::<u8>()?;
    if !(1..=VERSION).contains(&version) {
        return Err(Error::BadVersion(version));
    }
    let steam_id = c.read::<u64>()?;
    let timestamp = c.read::<u64>()?;
    let required = if version > 1 {
        parse_required(c)?
    } else {
        Vec::new()
    };
    Ok(Header {
        version,
        steam_id,
        timestamp,
        required,
        name: c.cstr(MAX_NAME)?.to_owned(),
        description: c.cstr(MAX_TEXT)?.to_owned(),
        author: c.cstr(MAX_NAME)?.to_owned(),
        addon_version: c.read::<i32>()?,
    })
}

fn parse_required(c: &mut Cursor) -> Result<Vec<String>> {
    let mut out = Vec::new();
    loop {
        let s = c.cstr(MAX_NAME)?;
        if s.is_empty() {
            return Ok(out);
        }
        out.push(s.to_owned());
    }
}
