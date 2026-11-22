/// Bare QC tokens (flex controller and desc names, event names) end at
/// whitespace, so anything outside an identifier charset is replaced to
/// keep a crafted name from adding commands.
pub fn ident(s: &str) -> String {
    let clean: String = s
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || "_.-".contains(c) {
                c
            } else {
                '_'
            }
        })
        .collect();
    if clean.is_empty() {
        "_".to_owned()
    } else {
        clean
    }
}

/// Key-value text is copied into a `{ }` block; unbalanced braces outside
/// quotes would close it early, so such text is dropped.
pub fn balanced(text: &str) -> bool {
    let (mut depth, mut quoted) = (0i64, false);
    for c in text.chars() {
        match c {
            '"' => quoted = !quoted,
            '{' if !quoted => depth += 1,
            '}' if !quoted => depth -= 1,
            _ => {}
        }
        if depth < 0 {
            return false;
        }
    }
    depth == 0 && !quoted
}

/// Game paths in the MDL keep Windows separators; studiomdl builds that
/// honour escapes (Jabroni Brawl, Crowbar issue #16) read `\m` as an
/// escape, while every build accepts `/`.
pub fn game_path(s: &str) -> String {
    super::fmt::quote(&s.replace('\\', "/"))
}
