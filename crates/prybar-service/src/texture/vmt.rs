//! Just enough VMT (KeyValues text) to find the base texture.

/// The `$basetexture` value, or the `include` target of a patch material
/// as `Err` so the caller can follow it once.
pub fn base_texture(text: &str) -> Option<Result<String, String>> {
    let mut include = None;
    let mut tokens = tokens(text);
    while let Some(key) = tokens.next() {
        if key.eq_ignore_ascii_case("$basetexture") {
            return tokens.next().map(|v| Ok(v.to_owned()));
        }
        if key.eq_ignore_ascii_case("include") {
            include = tokens.next().map(str::to_owned);
        }
    }
    include.map(Err)
}

/// Whether any of `keys` is set to a non-zero value (`$alphatest 1`).
pub fn flag(text: &str, keys: &[&str]) -> bool {
    let mut tokens = tokens(text);
    while let Some(key) = tokens.next() {
        if keys.iter().any(|k| key.eq_ignore_ascii_case(k)) {
            return tokens.next().is_some_and(|v| v.trim() != "0");
        }
    }
    false
}

/// Quoted or bare words; braces and `//` comments are dropped.
fn tokens(text: &str) -> impl Iterator<Item = &str> {
    text.lines()
        .map(|l| l.split("//").next().unwrap_or(""))
        .flat_map(|l| split(l).into_iter())
        .filter(|t| !matches!(*t, "{" | "}"))
}

fn split(line: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = line.trim_start();
    while !rest.is_empty() {
        let (tok, tail) = match rest.strip_prefix('"') {
            Some(q) => q.split_once('"').unwrap_or((q, "")),
            None => rest.split_at(
                rest.find(|c: char| c.is_whitespace() || c == '"')
                    .unwrap_or(rest.len()),
            ),
        };
        out.push(tok);
        rest = tail.trim_start();
    }
    out
}
