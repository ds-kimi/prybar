use super::pretty;
use crate::cli::{DecompileArgs, InspectArgs};
use crate::output::Output;
use prybar_service::model::{self, DecompileOptions, DecompileReport};
use prybar_service::{ExitCode, Result};
use std::fmt::Write;

pub fn inspect(a: &InspectArgs, out: &Output) -> Result<u8> {
    let summary = model::inspect(&a.model)?;
    out.emit(&summary, pretty);
    Ok(0)
}

pub fn decompile(a: &DecompileArgs, out: &Output) -> Result<u8> {
    let opts = DecompileOptions {
        out: a.out.clone(),
        recursive: a.recursive,
        split_qc: a.split_qc,
        flat: a.flat,
        with_includes: !a.no_includes,
        cancel: None,
    };
    let report = model::decompile(&a.inputs, &opts, &|m| {
        out.progress(&format!("decompiling {m}"))
    })?;
    out.emit(&report, text);
    Ok(code(&report))
}

/// Nothing decompiled: the failures' shared code (not found, invalid...)
/// or 4 when they differ. Some decompiled: 7, so scripts can tell "some
/// output is usable" from "none is".
fn code(r: &DecompileReport) -> u8 {
    let Some(first) = r.failures.first() else {
        return ExitCode::Ok as u8;
    };
    if !r.models.is_empty() {
        return ExitCode::Partial as u8;
    }
    match r.failures.iter().all(|f| f.code == first.code) {
        true => first.code,
        false => ExitCode::InvalidInput as u8,
    }
}

fn text(r: &DecompileReport) -> String {
    let mut s = String::new();
    for m in &r.models {
        let _ = writeln!(
            s,
            "ok   {} -> {} ({} files)",
            m.input,
            m.out_dir.display(),
            m.files
        );
        if let Some(from) = &m.included_from {
            let _ = writeln!(s, "     included by {from}");
        }
        for n in &m.notes {
            let _ = writeln!(s, "     note: {n}");
        }
    }
    for f in &r.failures {
        let _ = writeln!(s, "fail {}: {}", f.input, f.error);
    }
    for w in &r.warnings {
        let _ = writeln!(s, "warn {w}");
    }
    s
}
