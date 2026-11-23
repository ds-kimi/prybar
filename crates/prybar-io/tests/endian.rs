use prybar_io::{Cursor, Reader};

#[test]
fn big_endian_reader_swaps_every_scalar_width() {
    let data = [0, 0, 0, 16, 0x12, 0x34, 0x3f, 0x80, 0, 0, b'V', b'P'];
    let r = Reader::big_endian(&data);
    assert!(r.is_big_endian());
    assert_eq!(r.read::<i32>(0).unwrap(), 16);
    assert_eq!(r.read::<u16>(4).unwrap(), 0x1234);
    assert_eq!(r.read::<f32>(6).unwrap(), 1.0);
    assert_eq!(r.array::<u16>(4, 2, 2).unwrap(), [0x1234, 0x3f80]);
    assert_eq!(r.bytes(10, 2).unwrap(), b"VP");
    let mut c = Cursor::new(r, 0);
    assert_eq!(c.read::<u32>().unwrap(), 16);
    assert!(!Reader::new(&data).is_big_endian());
}
