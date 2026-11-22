use crate::case::CaseResult;
use std::fmt::Write;
use std::path::Path;

/// `roundtrip.json` for scripts and `roundtrip.md` for people.
pub fn write(dir: &Path, results: &[CaseResult]) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let json = serde_json::to_string_pretty(results).map_err(std::io::Error::other)?;
    std::fs::write(dir.join("roundtrip.json"), json)?;
    std::fs::write(dir.join("roundtrip.md"), markdown(results))
}

pub fn pass_count(results: &[CaseResult]) -> usize {
    results.iter().filter(|r| r.stage == "pass").count()
}

fn markdown(results: &[CaseResult]) -> String {
    let pass = pass_count(results);
    let mut s = String::from("# Round-trip report\n\n");
    let pct = 100.0 * pass as f64 / results.len().max(1) as f64;
    let _ = writeln!(s, "{pass} / {} models pass ({pct:.1}%).\n", results.len());
    s.push_str("| Model | Stage | Differences |\n|---|---|---|\n");
    for r in results.iter().filter(|r| r.stage != "pass") {
        let why = r.diffs.join("; ").replace('|', "\\|");
        let _ = writeln!(s, "| `{}` | {} | {} |", r.model, r.stage, why);
    }
    s
}
