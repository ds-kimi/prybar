use super::key_table::NAME_KEY;

/// Names are keyed by their absolute position in the file, so the same
/// name encodes differently at different offsets; `offset` must be the
/// file offset of `raw[0]`. XOR is its own inverse: this also encodes.
pub fn xor_name(raw: &[u8], offset: usize) -> Vec<u8> {
    let mask = NAME_KEY.len() - 1;
    raw.iter()
        .enumerate()
        .map(|(i, b)| b ^ NAME_KEY[offset.wrapping_add(i) & mask])
        .collect()
}

/// Backslashes are normalized so paths compare the same on every OS.
pub fn decode_name(raw: &[u8], offset: usize) -> String {
    String::from_utf8_lossy(&xor_name(raw, offset)).replace(char::from(92), "/")
}
