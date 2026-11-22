use crate::UpdateError;

/// Most lines a SHA256SUMS file may hold; a release has a handful.
const MAX_LINES: usize = 1024;

/// Parses `sha256sum` / `shasum -a 256` output: `<64 hex>  [*]<name>`.
/// Any malformed line rejects the whole file rather than guessing.
pub fn parse_sums(text: &str) -> Result<Vec<(String, [u8; 32])>, UpdateError> {
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.len() > MAX_LINES {
        return Err(UpdateError::BadRelease("SHA256SUMS too long".into()));
    }
    lines.into_iter().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<(String, [u8; 32]), UpdateError> {
    let bad = || UpdateError::BadRelease(format!("bad SHA256SUMS line: {:.80}", line));
    let (hash, rest) = line.split_once(char::is_whitespace).ok_or_else(bad)?;
    let name = rest.trim_start().trim_start_matches('*').trim_end();
    if hash.len() != 64 || name.is_empty() || name.contains(['/', '\\']) {
        return Err(bad());
    }
    let mut out = [0u8; 32];
    hex::decode_to_slice(hash, &mut out).map_err(|_| bad())?;
    Ok((name.to_string(), out))
}
