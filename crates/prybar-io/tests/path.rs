use prybar_io::{safe_join, Cursor, Reader};
use std::path::Path;

#[test]
fn safe_join_rejects_escapes() {
    let root = Path::new("out");
    assert_eq!(
        safe_join(root, "a/b.txt"),
        Some(root.join("a").join("b.txt"))
    );
    assert_eq!(safe_join(root, r"./a\b"), Some(root.join("a").join("b")));
    for bad in ["../x", "a/../../x", "C:/x", "c:x", "", "/", "a\0b"] {
        assert_eq!(safe_join(root, bad), None, "{bad:?}");
    }
}

#[test]
fn cursor_walks_and_stops_at_end() {
    let data = [b'h', b'i', 0, 7, 0];
    let mut c = Cursor::new(Reader::new(&data), 0);
    assert_eq!(c.cstr(16), Ok("hi"));
    assert_eq!(c.read::<u16>(), Ok(7));
    assert!(c.read::<u8>().is_err());
    assert_eq!(c.pos, 5);
}
