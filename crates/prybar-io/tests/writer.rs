use prybar_io::{Reader, Writer};

#[test]
fn round_trips_through_reader() {
    let mut w = Writer::new();
    w.put(7u32).put(-1i16).put_cstr("hi").align(4).put(1.5f32);
    assert_eq!(w.position(), 16);
    let bytes = w.into_inner();
    let r = Reader::new(&bytes);
    assert_eq!(r.read::<u32>(0), Ok(7));
    assert_eq!(r.read::<i16>(4), Ok(-1));
    assert_eq!(r.cstr(6, 8), Ok("hi"));
    assert_eq!(r.read::<f32>(12), Ok(1.5));
}

#[test]
fn patch_and_align_edge_cases() {
    let mut w = Writer::new();
    w.put(0u32).align(0);
    assert!(w.patch_u32(0, 9));
    assert!(!w.patch_u32(2, 9));
    assert!(!w.patch_u32(usize::MAX, 9));
    assert_eq!(w.into_inner(), vec![9, 0, 0, 0]);
}
