use prybar_io::{Error, Reader};

#[test]
fn reads_scalars_and_rejects_truncation() {
    let r = Reader::new(&[1, 0, 0, 0, 0xff]);
    assert_eq!(r.read::<u32>(0), Ok(1));
    assert_eq!(r.read::<u8>(4), Ok(0xff));
    assert!(matches!(r.read::<u32>(2), Err(Error::OutOfBounds { .. })));
    assert!(r.read::<u8>(usize::MAX).is_err());
    assert!(r.bytes(usize::MAX, 2).is_err());
}

#[test]
fn cstr_requires_terminator_within_cap() {
    let r = Reader::new(b"abc\0defg");
    assert_eq!(r.cstr(0, 16), Ok("abc"));
    assert!(matches!(r.cstr(4, 16), Err(Error::Unterminated { .. })));
    assert!(matches!(r.cstr(0, 2), Err(Error::Unterminated { .. })));
    assert!(r.cstr(100, 16).is_err());
    let bad = Reader::new(&[0xff, 0]);
    assert!(matches!(bad.cstr(0, 4), Err(Error::InvalidUtf8 { .. })));
}

#[test]
fn array_enforces_cap_and_bounds() {
    let r = Reader::new(&[1, 0, 2, 0, 3]);
    assert_eq!(r.array::<u16>(0, 2, 8), Ok(vec![1, 2]));
    assert!(matches!(
        r.array::<u16>(0, 9, 8),
        Err(Error::CountTooLarge { .. })
    ));
    assert!(matches!(
        r.array::<u16>(0, 3, 8),
        Err(Error::OutOfBounds { .. })
    ));
    assert!(r.array::<u64>(0, usize::MAX, usize::MAX).is_err());
}

#[test]
fn empty_buffer_is_safe() {
    let r = Reader::new(&[]);
    assert!(r.is_empty());
    assert!(r.read::<u8>(0).is_err());
    assert_eq!(r.array::<u32>(0, 0, 0), Ok(vec![]));
}
