use super::buffer::{Buffer, Op};
use super::step::{step, Dir};

fn typed(text: &str) -> Buffer {
    let mut b = Buffer::default();
    b.apply(Op::Insert(text.into()));
    b
}

/// Shift-arrows grow a selection, typing replaces it, undo brings it back.
#[test]
fn select_replace_undo() {
    let mut b = typed("héllo");
    b.apply(Op::Move(Dir::Left, true));
    b.apply(Op::Move(Dir::Left, true));
    assert_eq!(&b.text[b.range()], "lo");
    assert!(b.apply(Op::Insert("p!".into())));
    assert_eq!(b.text, "hélp!");
    assert!(b.apply(Op::Undo));
    assert_eq!(b.text, "héllo");
    b.apply(Op::Move(Dir::Home, false));
    b.apply(Op::Move(Dir::Right, false));
    b.apply(Op::Forward);
    assert_eq!(b.text, "hllo", "a two-byte char goes in one step");
}

/// Up and down keep the column and clamp to shorter lines.
#[test]
fn lines_keep_column() {
    let text = "abcd\nx\nwxyz";
    assert_eq!(step(text, 3, Dir::Down), 6);
    assert_eq!(step(text, 6, Dir::Down), 8);
    assert_eq!(step(text, 10, Dir::Up), 6);
    assert_eq!(step(text, 2, Dir::Up), 0);
    assert_eq!(step(text, 9, Dir::Down), text.len());
    assert_eq!(step(text, 6, Dir::Home), 5);
}

/// Odd offsets never split a char or run past the end.
#[test]
fn offsets_are_clamped() {
    let mut b = typed("añb");
    b.select_to(2, false);
    assert_eq!(b.head, 1);
    b.select_to(99, true);
    assert_eq!(b.head, b.text.len());
    assert!(b.apply(Op::Back));
    assert_eq!(b.text, "a", "backspace takes the selection");
}
