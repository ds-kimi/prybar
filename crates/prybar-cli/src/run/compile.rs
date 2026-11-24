use crate::cli::CompileArgs;
use crate::output::Output;
use prybar_service::{tools, ExitCode, Result};
use std::path::Path;
use std::sync::atomic::AtomicBool;

/// A compile that ran but failed exits 6 with the report on stdout, so
/// the errors are not lost behind a bare error line.
pub fn compile(a: &CompileArgs, out: &Output) -> Result<u8> {
    let single = match a.inputs.as_slice() {
        [one] if !a.recursive && Path::new(one).is_file() => Path::new(one),
        _ => return compile_batch(a, out),
    };
    out.progress(&format!("compiling {}", single.display()));
    let report = tools::compile(&a.game, single)?;
    out.emit(&report, |r| {
        let mut s = r.log.clone();
        s.push_str(if r.success {
            "\ncompile ok\n"
        } else {
            "\ncompile FAILED\n"
        });
        s
    });
    Ok(if report.success {
        0
    } else {
        ExitCode::Tool as u8
    })
}

/// Same exit rule as decompile: 7 when some QCs compiled, otherwise the
/// failures' shared code (6 compile failed, 3 not found) or 4 if mixed.
fn compile_batch(a: &CompileArgs, out: &Output) -> Result<u8> {
    let opts = tools::CompileOptions {
        recursive: a.recursive,
        jobs: a.jobs,
    };
    let progress = |qc: &str| out.progress(&format!("compiling {qc}"));
    let never = AtomicBool::new(false);
    let r = tools::compile_batch(&a.game, &a.inputs, &opts, &progress, &never)?;
    out.emit(&r, batch_text);
    let bad: Vec<u8> = r
        .items
        .iter()
        .filter(|i| i.code != 0)
        .map(|i| i.code)
        .collect();
    Ok(match bad.first() {
        None => 0,
        Some(_) if r.compiled > 0 => ExitCode::Partial as u8,
        Some(&c) if bad.iter().all(|&b| b == c) => c,
        Some(_) => ExitCode::InvalidInput as u8,
    })
}

fn batch_text(r: &tools::CompileBatchReport) -> String {
    let mut s = String::new();
    for i in &r.items {
        let why = i
            .error
            .as_deref()
            .or(i.errors.first().map(|l| l.text.as_str()));
        let status = match i.status {
            tools::ItemStatus::Ok => "ok",
            tools::ItemStatus::Failed => "FAILED",
            tools::ItemStatus::Cancelled => "cancelled",
        };
        s.push_str(&format!("{status}\t{}", i.qc));
        if let Some(w) = why.filter(|_| i.code != 0) {
            s.push_str(&format!("\n  {w}"));
        }
        s.push('\n');
    }
    s.push_str(&format!(
        "{} compiled, {} failed, {} cancelled\n",
        r.compiled, r.failed, r.cancelled
    ));
    s
}
