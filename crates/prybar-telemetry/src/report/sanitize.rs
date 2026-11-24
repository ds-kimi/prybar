//! Strips everything that could name a person, a path or a file.

use crate::event::{MAX_FRAMES, MAX_MESSAGE};

/// Panic text with quoted spans, paths, file names and the user name
/// replaced by placeholders, cut to `MAX_MESSAGE`.
pub fn message(raw: &str, user: Option<&str>) -> String {
    let mut text = raw.to_string();
    if let Some(u) = user.filter(|u| u.len() >= 2) {
        text = text.replace(u, "<user>");
    }
    let words: Vec<String> = unquote(&text).split_whitespace().map(word).collect();
    let mut out = words.join(" ");
    if out.len() > MAX_MESSAGE {
        let mut cut = MAX_MESSAGE;
        while !out.is_char_boundary(cut) {
            cut -= 1;
        }
        out.truncate(cut);
    }
    out
}

/// Quoted text is usually a model, material or file name, so it goes.
fn unquote(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(i) = rest.find(['"', '\'', '`']) {
        out.push_str(&rest[..i]);
        let quote = rest[i..].chars().next().unwrap_or('"');
        let after = &rest[i + 1..];
        match after.find(quote) {
            Some(j) => {
                out.push_str("<redacted>");
                rest = &after[j + 1..];
            }
            None => {
                out.push(quote);
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

fn word(w: &str) -> String {
    if w.contains(['/', '\\']) || w.contains(":\\") {
        return "<path>".into();
    }
    // Emails, IP addresses and long numbers (SteamID64, workshop ids,
    // phone numbers) can point at a person; short numbers stay for context.
    if w.contains('@') {
        return "<email>".into();
    }
    let bare = w.trim_end_matches([',', '.', ';', ':', ')', ']']);
    let digits = bare.bytes().filter(u8::is_ascii_digit).count();
    let dots = bare.bytes().filter(|&b| b == b'.').count();
    if digits >= 7 || (dots == 3 && bare.bytes().all(|b| b.is_ascii_digit() || b == b'.')) {
        return "<number>".into();
    }
    let is_file = bare.rsplit_once('.').is_some_and(|(stem, ext)| {
        !stem.is_empty()
            && (1..=5).contains(&ext.len())
            && ext.chars().all(|c| c.is_ascii_alphabetic())
    });
    if is_file {
        "<file>".into()
    } else {
        w.into()
    }
}

/// Symbol names of prybar's own frames from a `std::backtrace` dump,
/// without the hash suffix; std, GPUI and file locations are dropped.
pub fn frames(backtrace: &str) -> Vec<String> {
    backtrace
        .lines()
        .filter_map(|l| l.trim().split_once(": "))
        .filter(|(n, _)| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
        .map(|(_, sym)| sym.trim())
        .filter(|sym| sym.starts_with("prybar"))
        .map(|sym| match sym.rsplit_once("::h") {
            Some((head, hash)) if hash.bytes().all(|b| b.is_ascii_hexdigit()) => head,
            _ => sym,
        })
        .filter(|sym| sym.len() <= 200)
        .take(MAX_FRAMES)
        .map(String::from)
        .collect()
}
