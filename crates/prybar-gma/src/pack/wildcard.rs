/// gmad-style glob: `*` matches any run of characters, `/` included, so
/// `lua/*.lua` also covers nested folders. Iterative with a single
/// backtrack point, so hostile patterns stay linear-ish instead of exponential.
pub(crate) fn wildcard(pattern: &str, text: &str) -> bool {
    let (p, s) = (pattern.as_bytes(), text.as_bytes());
    let (mut pi, mut si) = (0, 0);
    let mut star: Option<(usize, usize)> = None;
    while si < s.len() {
        if p.get(pi) == Some(&b'*') {
            star = Some((pi, si));
            pi += 1;
        } else if p.get(pi) == Some(&s[si]) {
            pi += 1;
            si += 1;
        } else if let Some((sp, ss)) = star {
            pi = sp + 1;
            si = ss + 1;
            star = Some((sp, ss + 1));
        } else {
            return false;
        }
    }
    p[pi..].iter().all(|&c| c == b'*')
}
