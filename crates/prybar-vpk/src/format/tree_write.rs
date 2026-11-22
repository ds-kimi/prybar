use crate::entry::split_path;
use crate::{Entry, Error, Result};
use prybar_io::Writer;
use std::collections::BTreeMap;

type Files<'a> = Vec<(&'a str, &'a Entry)>;

pub(crate) fn write_tree(entries: &[Entry]) -> Result<Vec<u8>> {
    let mut tree: BTreeMap<&str, BTreeMap<&str, Files>> = BTreeMap::new();
    for e in entries {
        let (ext, dir, name) = split_path(&e.path);
        let files = tree.entry(ext).or_default().entry(dir).or_default();
        files.push((name, e));
    }
    let mut w = Writer::new();
    for (ext, dirs) in &tree {
        w.put_cstr(ext);
        for (dir, files) in dirs {
            w.put_cstr(dir);
            for (name, e) in files {
                w.put_cstr(name);
                write_entry(&mut w, e)?;
            }
            w.put(0u8);
        }
        w.put(0u8);
    }
    w.put(0u8);
    Ok(w.into_inner())
}

fn write_entry(w: &mut Writer, e: &Entry) -> Result<()> {
    let preload_len = u16::try_from(e.preload.len()).map_err(|_| Error::TooLarge)?;
    w.put(e.crc).put(preload_len).put(e.archive_index);
    w.put(e.offset).put(e.length).put(0xffffu16);
    w.put_bytes(&e.preload);
    Ok(())
}
