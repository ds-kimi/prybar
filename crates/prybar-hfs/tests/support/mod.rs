use prybar_hfs::xor_name;
use prybar_io::Writer;

/// Names are encoded with the key at their own offset.
fn name(w: &mut Writer, n: &str) {
    let at = w.position();
    w.put_bytes(&xor_name(n.as_bytes(), at));
}

fn local(w: &mut Writer, n: &str, data: &[u8]) -> u32 {
    let at = w.position() as u32;
    w.put(0x0403_4648u32)
        .put_bytes(&[0u8; 14])
        .put(data.len() as u32)
        .put(data.len() as u32);
    w.put(n.len() as u16).put(0u16);
    name(w, n);
    w.put_bytes(data);
    at
}

/// Synthetic package: local headers + data, central directory, EOCD.
pub fn build(files: &[(&str, &[u8])]) -> Vec<u8> {
    let mut w = Writer::new();
    let locals: Vec<u32> = files.iter().map(|(n, d)| local(&mut w, n, d)).collect();
    let dir = w.position() as u32;
    for (i, (n, data)) in files.iter().enumerate() {
        w.put(0x0201_4648u32)
            .put_bytes(&[0u8; 16])
            .put(data.len() as u32)
            .put(data.len() as u32);
        w.put(n.len() as u16).put_bytes(&[0u8; 12]).put(locals[i]);
        name(&mut w, n);
    }
    let size = w.position() as u32 - dir;
    let count = files.len() as u16;
    w.put(0x0605_4648u32)
        .put(0u32)
        .put(count)
        .put(count)
        .put(size)
        .put(dir)
        .put(0u16);
    w.into_inner()
}
