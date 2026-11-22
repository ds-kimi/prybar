//! Builds synthetic VTF files so tests and fuzz seeds never ship game data.
#![allow(dead_code)]

/// A VTF with no thumbnail; `data` is the image data, smallest mip first.
pub fn vtf(minor: u32, format: i32, size: (u16, u16), mips: u8, data: &[u8]) -> Vec<u8> {
    let header: u32 = if minor >= 3 { 88 } else { 80 };
    let mut b = Vec::with_capacity(header as usize + data.len());
    b.extend_from_slice(b"VTF\0");
    for v in [7, minor, header] {
        b.extend_from_slice(&v.to_le_bytes());
    }
    b.extend_from_slice(&size.0.to_le_bytes());
    b.extend_from_slice(&size.1.to_le_bytes());
    b.extend_from_slice(&0u32.to_le_bytes()); // flags
    b.extend_from_slice(&1u16.to_le_bytes()); // frames
    b.extend_from_slice(&0u16.to_le_bytes()); // first frame
    b.resize(52, 0);
    b.extend_from_slice(&format.to_le_bytes());
    b.push(mips);
    b.extend_from_slice(&(-1i32).to_le_bytes()); // no thumbnail
    b.extend_from_slice(&[0, 0]);
    b.extend_from_slice(&1u16.to_le_bytes()); // depth
    b.resize(68, 0);
    b.extend_from_slice(&u32::from(minor >= 3).to_le_bytes());
    b.resize(80, 0);
    if minor >= 3 {
        b.extend_from_slice(&[0x30, 0, 0, 0]);
        b.extend_from_slice(&header.to_le_bytes());
    }
    b.extend_from_slice(data);
    b
}

/// A 4x4 DXT1 block: solid red on the left half, solid blue on the right.
pub fn dxt1_red_blue() -> [u8; 8] {
    let (red, blue) = (0xf800u16, 0x001fu16);
    let mut b = [0u8; 8];
    b[..2].copy_from_slice(&red.to_le_bytes());
    b[2..4].copy_from_slice(&blue.to_le_bytes());
    // Index 0 (red) for x < 2, index 1 (blue) for x >= 2, every row.
    b[4..].copy_from_slice(&0x5050_5050u32.to_le_bytes());
    b
}
