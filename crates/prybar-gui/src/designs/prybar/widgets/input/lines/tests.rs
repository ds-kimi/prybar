use super::{row_of, wrap_line, x_at};

/// Ten pixels a byte, like a monospace font.
fn mono(i: usize) -> f32 {
    i as f32 * 10.
}

/// Lines break after a space, never before one, and a long word is cut.
#[test]
fn wraps_at_spaces() {
    let rows = wrap_line("one two three", 75., mono);
    assert_eq!(rows, vec![0..8, 8..13]);
    assert_eq!(wrap_line("abcdefghij", 40., mono), vec![0..4, 4..8, 8..10]);
    assert_eq!(wrap_line("a    b", 20., mono), vec![0..5, 5..6]);
    assert_eq!(wrap_line("", 20., mono), vec![0..0]);
    assert_eq!(wrap_line("fits", 400., mono), vec![0..4]);
}

/// A wrap point belongs to the row after it; a line end to its own row.
#[test]
fn offsets_find_their_row() {
    let rows = [0..4, 4..8, 9..12];
    assert_eq!(row_of(&rows, 3), 0);
    assert_eq!(row_of(&rows, 4), 1);
    assert_eq!(row_of(&rows, 8), 1);
    assert_eq!(row_of(&rows, 12), 2);
    let xs = [(0, 0.), (1, 7.), (3, 20.)];
    assert_eq!((x_at(&xs, 1), x_at(&xs, 2), x_at(&xs, 9)), (7., 20., 20.));
}
