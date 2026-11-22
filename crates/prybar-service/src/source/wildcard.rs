/// Case-insensitive `*` / `?` match; `*` also crosses `/` (callers that
/// want one folder level check the depth themselves).
pub fn matches(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.to_lowercase().chars().collect();
    let t: Vec<char> = text.to_lowercase().chars().collect();
    let (mut pi, mut ti) = (0, 0);
    let mut star: Option<(usize, usize)> = None;
    while ti < t.len() {
        // Star first: a literal `*` in the text must not consume it.
        if pi < p.len() && p[pi] == '*' {
            star = Some((pi, ti));
            pi += 1;
        } else if pi < p.len() && (p[pi] == '?' || p[pi] == t[ti]) {
            pi += 1;
            ti += 1;
        } else if let Some((sp, st)) = star {
            // Backtracking only to the last star keeps this linear-ish
            // instead of exponential on hostile patterns.
            pi = sp + 1;
            ti = st + 1;
            star = Some((sp, st + 1));
        } else {
            return false;
        }
    }
    p[pi..].iter().all(|&c| c == '*')
}

pub fn has_wildcard(s: &str) -> bool {
    s.contains(['*', '?'])
}

#[cfg(test)]
mod tests {
    use super::matches;

    #[test]
    fn stars_and_marks() {
        assert!(matches("models/*.mdl", "Models/a/B.MDL"));
        assert!(matches("a?c", "abc"));
        assert!(!matches("*.mdl", "a.vvd"));
        assert!(matches("*", ""));
        assert!(!matches("a*b*c", "aXbY"));
        assert!(matches("*b", "*ab"));
    }
}
