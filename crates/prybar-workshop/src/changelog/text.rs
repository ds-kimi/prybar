/// The text after the first `open` up to the next `close`.
pub fn between<'a>(s: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = s.find(open)? + open.len();
    let len = s[start..].find(close)?;
    Some(&s[start..start + len])
}

/// Steam puts the update's unix time in the note's `id` attribute.
pub fn stamp(body: &str) -> Option<u64> {
    let p = between(body, "<p", ">")?;
    let id = between(p, "id=\"", "\"")?;
    let digits = id.trim();
    if digits.is_empty() || digits.len() > 12 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok()
}

/// Tags removed (`<br>` becomes a line break), the common entities
/// decoded, whitespace tidied, cut at `max` chars.
pub fn plain(html: &str, max: usize) -> String {
    let mut out = String::new();
    let mut rest = html;
    while let Some(lt) = rest.find('<') {
        out.push_str(&rest[..lt]);
        let Some(gt) = rest[lt..].find('>') else {
            rest = "";
            break;
        };
        let tag = rest[lt + 1..lt + gt]
            .trim_start_matches('/')
            .to_ascii_lowercase();
        if tag.starts_with("br") || tag.starts_with("li") || tag == "p" {
            out.push('\n');
        }
        rest = &rest[lt + gt + 1..];
    }
    out.push_str(rest);
    let decoded = decode(&out);
    let lines: Vec<&str> = decoded.lines().map(str::trim).collect();
    let tidy = lines.join("\n").trim().to_owned();
    tidy.chars().take(max).collect()
}

fn decode(s: &str) -> String {
    [
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&#39;", "'"),
        ("&nbsp;", " "),
        ("&amp;", "&"),
    ]
    .iter()
    .fold(s.to_owned(), |acc, (from, to)| acc.replace(from, to))
}
