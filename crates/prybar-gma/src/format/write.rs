use super::{Header, MAGIC};
use prybar_io::Writer;

/// `files` are (archive path, contents) in output order. The trailing CRC
/// covers everything before it; gmod ignores it but it helps detect
/// truncated downloads.
pub(crate) fn encode(h: &Header, files: &[(String, Vec<u8>)]) -> Vec<u8> {
    let mut w = Writer::new();
    w.put_bytes(MAGIC)
        .put(h.version)
        .put(h.steam_id)
        .put(h.timestamp);
    for r in &h.required {
        w.put_cstr(r);
    }
    w.put(0u8);
    w.put_cstr(&h.name)
        .put_cstr(&h.description)
        .put_cstr(&h.author);
    w.put(h.addon_version);
    for (i, (path, data)) in files.iter().enumerate() {
        w.put(i as u32 + 1).put_cstr(path);
        w.put(data.len() as i64).put(crc32fast::hash(data));
    }
    w.put(0u32);
    for (_, data) in files {
        w.put_bytes(data);
    }
    let mut out = w.into_inner();
    let crc = crc32fast::hash(&out);
    out.extend_from_slice(&crc.to_le_bytes());
    out
}
