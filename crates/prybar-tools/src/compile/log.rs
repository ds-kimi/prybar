use super::LogLine;

/// Lines starting like this are diagnostics; studiomdl has no other
/// structured channel.
const ERROR: &str = "ERROR";
const WARNING: &str = "WARNING";

/// Splits a studiomdl log into (errors, warnings). Matching is on the
/// line start, case-insensitive: studiomdl writes `ERROR:`, `Error:` and
/// `WARNING:`, while words like "errors" mid-line are plain verbose text.
pub fn parse_log(log: &str) -> (Vec<LogLine>, Vec<LogLine>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    for (i, raw) in log.lines().enumerate() {
        let text = raw.trim();
        let entry = || LogLine {
            line: i + 1,
            text: text.to_string(),
        };
        if starts_with(text, ERROR) {
            errors.push(entry());
        } else if starts_with(text, WARNING) {
            warnings.push(entry());
        }
    }
    (errors, warnings)
}

fn starts_with(text: &str, word: &str) -> bool {
    text.get(..word.len())
        .is_some_and(|head| head.eq_ignore_ascii_case(word))
        && text
            .get(word.len()..)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with([':', ' ', '!']))
}

/// Files studiomdl says it wrote (`writing <path>:` lines in verbose
/// mode), so a caller can point at the compiled model.
pub fn written_files(log: &str) -> Vec<String> {
    const EXTS: [&str; 6] = [".mdl", ".vvd", ".vtx", ".phy", ".ani", ".cvvd"];
    log.lines()
        .filter_map(|l| {
            let l = l.trim();
            let rest = l
                .get(..8)?
                .eq_ignore_ascii_case("writing ")
                .then(|| &l[8..])?;
            let path = rest.trim().trim_end_matches(':').trim();
            let lower = path.to_ascii_lowercase();
            EXTS.iter()
                .any(|e| lower.ends_with(e))
                .then(|| path.to_owned())
        })
        .collect()
}
