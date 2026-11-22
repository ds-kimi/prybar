use crate::{Error, Result};
use prybar_io::Reader;
use serde::Serialize;

/// Groups versions that share one on-disk layout, so one parser serves
/// the whole family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Family {
    GoldSrc,
    SourceLegacy,
    Source44,
    Source52,
    Source53,
    /// `IDSQ`: a GoldSrc external sequence group, not a model on its own.
    SequenceGroup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Version {
    pub number: i32,
    pub family: Family,
}

/// Reads the magic and version field without trusting anything else.
pub fn detect(data: &[u8]) -> Result<Version> {
    let r = Reader::new(data);
    let magic: [u8; 4] = r.bytes(0, 4)?.try_into().expect("4 bytes");
    let number: i32 = r.read(4)?;
    let family = match &magic {
        b"IDST" => family_of(number)?,
        b"IDSQ" => Family::SequenceGroup,
        _ => return Err(Error::BadMagic(magic)),
    };
    Ok(Version { number, family })
}

fn family_of(number: i32) -> Result<Family> {
    Ok(match number {
        4 | 6 | 10 | 14 => Family::GoldSrc,
        2531 | 27..=37 => Family::SourceLegacy,
        44..=49 => Family::Source44,
        52 => Family::Source52,
        53 => Family::Source53,
        _ => return Err(Error::Unsupported { version: number }),
    })
}
