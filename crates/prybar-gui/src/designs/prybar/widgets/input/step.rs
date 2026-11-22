//! Caret steps and the keys that edit a field.

use gpui::Keystroke;

use super::buffer::Op;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
}

/// The offset one step from `from`, by whole chars; Home and End and
/// Up and Down stay inside the current line.
pub fn step(text: &str, from: usize, dir: Dir) -> usize {
    let from = from.min(text.len());
    let start = text[..from].rfind('\n').map_or(0, |i| i + 1);
    let end = text[from..].find('\n').map_or(text.len(), |i| from + i);
    match dir {
        Dir::Left => text[..from].char_indices().last().map_or(0, |(i, _)| i),
        Dir::Right => text[from..]
            .chars()
            .next()
            .map_or(from, |c| from + c.len_utf8()),
        Dir::Home => start,
        Dir::End => end,
        Dir::Up if start == 0 => 0,
        Dir::Up => column(text, start - 1, from - start, true),
        Dir::Down if end == text.len() => end,
        Dir::Down => column(text, end + 1, from - start, false),
    }
}

/// The same column on the line before (`at` is its last byte) or after
/// (`at` is its first byte), clamped to that line and to a char edge.
fn column(text: &str, at: usize, col: usize, before: bool) -> usize {
    let start = if before {
        text[..at].rfind('\n').map_or(0, |i| i + 1)
    } else {
        at
    };
    let end = text[start..].find('\n').map_or(text.len(), |i| start + i);
    let mut to = (start + col).min(end);
    while !text.is_char_boundary(to) {
        to -= 1;
    }
    to
}

/// The edit a key means. Clipboard keys are handled by the input, since
/// they need the app; `None` lets the key go on to the app.
pub fn key_op(k: &Keystroke, multiline: bool) -> Option<Op> {
    let m = &k.modifiers;
    let cmd = if cfg!(target_os = "macos") {
        m.platform
    } else {
        m.control
    };
    let dir = |d| Some(Op::Move(d, m.shift));
    match k.key.as_str() {
        "a" if cmd => Some(Op::SelectAll),
        "z" if cmd => Some(Op::Undo),
        _ if cmd || m.control => None,
        "left" => dir(Dir::Left),
        "right" => dir(Dir::Right),
        "home" => dir(Dir::Home),
        "end" => dir(Dir::End),
        "up" if multiline => dir(Dir::Up),
        "down" if multiline => dir(Dir::Down),
        "backspace" => Some(Op::Back),
        "delete" => Some(Op::Forward),
        "enter" if multiline => Some(Op::Insert("\n".into())),
        _ => k
            .key_char
            .clone()
            .filter(|c| !c.is_empty() && !c.chars().any(char::is_control))
            .map(Op::Insert),
    }
}

/// The rule of fields that take any text.
pub fn any(_: &str) -> Option<&'static str> {
    None
}
