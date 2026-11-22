use std::path::PathBuf;

/// Caps keep a hostile or corrupt vdf from costing more than a glance.
const MAX_BYTES: usize = 1 << 20;
const MAX_PATHS: usize = 256;

/// Library folders listed in a `libraryfolders.vdf` (KeyValues text).
/// Newer files hold `"path" "<dir>"` per library block; older ones list
/// `"<n>" "<dir>"` directly. Numeric keys with numeric values (app ids and
/// sizes in the newer `apps` block) are not paths.
pub fn library_paths(bytes: &[u8]) -> Vec<PathBuf> {
    let text = String::from_utf8_lossy(&bytes[..bytes.len().min(MAX_BYTES)]);
    let tokens = tokens(&text);
    let mut out = Vec::new();
    let mut i = 0;
    // Walk key/value pairs; a key followed by a brace opens a block and
    // takes no value, so a value equal to "path" is never read as a key.
    while i + 1 < tokens.len() && out.len() < MAX_PATHS {
        match (&tokens[i], &tokens[i + 1]) {
            (Some(key), Some(val)) => {
                if is_library(key, val) {
                    out.push(PathBuf::from(unescape(val)));
                }
                i += 2;
            }
            _ => i += 1,
        }
    }
    out
}

fn is_library(key: &str, val: &str) -> bool {
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    let path_key = key.eq_ignore_ascii_case("path") || digits(key);
    path_key && !val.is_empty() && !digits(val)
}

/// Quoted strings as `Some`, braces as `None`; everything else skipped.
fn tokens(text: &str) -> Vec<Option<String>> {
    let mut out = Vec::new();
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '{' || c == '}' {
            out.push(None);
        }
        if c != '"' {
            continue;
        }
        let mut tok = String::new();
        let mut escaped = false;
        for c in chars.by_ref() {
            if c == '"' && !escaped {
                break;
            }
            escaped = c == '\\' && !escaped;
            tok.push(c);
        }
        out.push(Some(tok));
    }
    out
}

/// Steam escapes backslashes and quotes inside values.
fn unescape(raw: &str) -> String {
    raw.replace("\\\\", "\\").replace("\\\"", "\"")
}
