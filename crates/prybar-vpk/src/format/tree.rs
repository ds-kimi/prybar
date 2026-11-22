use crate::entry::join_path;
use crate::{Entry, Error, Result};
use prybar_io::{Cursor, Reader};

/// Generous cap for one tree string; real names are far shorter.
const MAX_STR: usize = 4096;
const TERMINATOR: u16 = 0xffff;

/// `tree` must be sliced to the header + tree region so a forged tree
/// cannot read into file data.
pub(crate) fn parse_tree(tree: Reader, start: usize) -> Result<Vec<Entry>> {
    let mut c = Cursor::new(tree, start);
    let mut out = Vec::new();
    loop {
        let ext = c.cstr(MAX_STR)?;
        if ext.is_empty() {
            return Ok(out);
        }
        loop {
            let dir = c.cstr(MAX_STR)?;
            if dir.is_empty() {
                break;
            }
            loop {
                let name = c.cstr(MAX_STR)?;
                if name.is_empty() {
                    break;
                }
                out.push(parse_entry(&mut c, join_path(ext, dir, name))?);
            }
        }
    }
}

fn parse_entry(c: &mut Cursor, path: String) -> Result<Entry> {
    let crc = c.read::<u32>()?;
    let preload_len = c.read::<u16>()?;
    let archive_index = c.read::<u16>()?;
    let offset = c.read::<u32>()?;
    let length = c.read::<u32>()?;
    if c.read::<u16>()? != TERMINATOR {
        return Err(Error::BadTerminator(c.pos - 2));
    }
    let preload = c.bytes(usize::from(preload_len))?.to_vec();
    Ok(Entry {
        path,
        crc,
        preload,
        archive_index,
        offset,
        length,
    })
}
