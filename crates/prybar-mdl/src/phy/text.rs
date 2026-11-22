use crate::ir::KvBlock;
use crate::read::Source;
use crate::Result;

/// The text section runs to the first NUL or the end of the file.
pub fn section(r: &Source, at: usize) -> Result<String> {
    let rest = r.bytes(at.min(r.len()), r.len().saturating_sub(at))?;
    let end = rest.iter().position(|&b| b == 0).unwrap_or(rest.len());
    // Tokens and blocks built from it cost a few times its size; the
    // worst case stays linear in the text, so no exact charge is needed.
    r.charge(end.saturating_mul(3))?;
    Ok(String::from_utf8_lossy(&rest[..end]).into_owned())
}

/// Top-level `name { "key" "value" ... }` blocks; nested braces inside a
/// block are skipped since studiomdl never writes them.
pub fn blocks(text: &str) -> Vec<KvBlock> {
    let t = tokens(text);
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < t.len() {
        if brace(t[i]) || !(brace(t[i + 1]) && t[i + 1].0 == "{") {
            i += 1;
            continue;
        }
        let mut block = KvBlock {
            name: t[i].0.to_owned(),
            pairs: Vec::new(),
        };
        i += 2;
        while i < t.len() && !(brace(t[i]) && t[i].0 == "}") {
            match t.get(i + 1).filter(|v| !brace(**v) && !brace(t[i])) {
                Some(v) => {
                    block.pairs.push((t[i].0.to_owned(), v.0.to_owned()));
                    i += 2;
                }
                None => i += 1,
            }
        }
        out.push(block);
        i += 1;
    }
    out
}

/// A quoted `"}"` is a value, not the end of a block.
fn brace((s, quoted): (&str, bool)) -> bool {
    !quoted && (s == "{" || s == "}")
}

/// Slices of the text, so tokenizing allocates one vector only; the flag
/// marks quoted tokens.
fn tokens(text: &str) -> Vec<(&str, bool)> {
    let mut out = Vec::new();
    let mut rest = text.trim_start();
    while let Some(c) = rest.chars().next() {
        let (token, after) = match c {
            '{' | '}' => rest.split_at(1),
            '"' => {
                let body = &rest[1..];
                let end = body.find('"').unwrap_or(body.len());
                (&body[..end], body.get(end + 1..).unwrap_or(""))
            }
            _ => {
                let end = rest.find(|n: char| n.is_whitespace() || "{}\"".contains(n));
                rest.split_at(end.unwrap_or(rest.len()))
            }
        };
        out.push((token, c == '"'));
        rest = after.trim_start();
    }
    out
}
