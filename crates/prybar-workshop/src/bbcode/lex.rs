//! Cutting a description into text and tags, one token at a time.

use super::Tag;

/// A tag longer than this is text: real ones are short, and the cap
/// keeps a stray `[` from scanning the whole input.
const MAX_TAG: usize = 512;

pub enum Tok<'a> {
    Text(&'a str),
    Open(Tag, Option<String>),
    Close(Tag),
}

/// The token at byte `at` and where the next one starts; `None` at the end.
pub fn next(s: &str, at: usize) -> Option<(Tok<'_>, usize)> {
    let rest = s.get(at..).filter(|r| !r.is_empty())?;
    if let Some((tok, len)) = tag(rest) {
        return Some((tok, at + len));
    }
    // A `[` that is no tag is text; the run goes on to the next `[`.
    let first = rest.chars().next().map_or(1, char::len_utf8);
    let end = rest[first..].find('[').map_or(rest.len(), |i| first + i);
    Some((Tok::Text(&rest[..end]), at + end))
}

/// A known tag at the very start of `rest`, with its length in bytes.
fn tag(rest: &str) -> Option<(Tok<'_>, usize)> {
    let body = rest.strip_prefix('[')?;
    let (end, c) = body
        .char_indices()
        .take(MAX_TAG)
        .find(|(_, c)| matches!(c, ']' | '[' | '\n'))?;
    if c != ']' {
        return None;
    }
    let inner = &body[..end];
    let len = end + 2;
    if let Some(name) = inner.strip_prefix('/') {
        return Some((Tok::Close(Tag::named(name.trim())?), len));
    }
    let (name, value) = split(inner);
    Some((Tok::Open(Tag::named(name)?, value), len))
}

/// `url=https://…` gives the value after `=`, quotes dropped;
/// `table noborder=1` keeps the options after the space as they are.
fn split(inner: &str) -> (&str, Option<String>) {
    let cut = inner.find(['=', ' ']).unwrap_or(inner.len());
    let (name, rest) = inner.split_at(cut);
    let value = match rest.strip_prefix('=') {
        Some(v) => v.trim().trim_matches(['"', '\'']),
        None => rest.trim(),
    };
    (name, (!value.is_empty()).then(|| value.to_owned()))
}
