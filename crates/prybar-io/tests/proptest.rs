//! Writer/Reader round-trip over mixed primitives and C strings.

use proptest::prelude::*;
use prybar_io::{Cursor, Reader, Writer};

proptest! {
    #[test]
    fn primitives_roundtrip(a: u8, b: i16, c: u32, d: i64, f: f32, s in "[a-zA-Z0-9 ]{0,64}") {
        let mut w = Writer::new();
        w.put(a).put(b).put(c).put(d).put(f).put_cstr(&s);
        let data = w.into_inner();
        let mut cur = Cursor::new(Reader::new(&data), 0);
        prop_assert_eq!(cur.read::<u8>().unwrap(), a);
        prop_assert_eq!(cur.read::<i16>().unwrap(), b);
        prop_assert_eq!(cur.read::<u32>().unwrap(), c);
        prop_assert_eq!(cur.read::<i64>().unwrap(), d);
        prop_assert_eq!(cur.read::<f32>().unwrap().to_bits(), f.to_bits());
        prop_assert_eq!(cur.cstr(256).unwrap(), s.as_str());
        prop_assert!(cur.read::<u8>().is_err());
    }

    #[test]
    fn reads_never_panic(data in prop::collection::vec(any::<u8>(), 0..64), off: usize, len: usize) {
        let r = Reader::new(&data);
        let _ = r.bytes(off, len);
        let _ = r.read::<u64>(off);
        let _ = r.cstr(off, len);
        let _ = r.array::<u32>(off, len, 1 << 20);
    }
}
