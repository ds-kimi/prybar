use super::pretty;
use crate::cli::{ListArgs, PackArgs, UnpackArgs};
use crate::output::Output;
use prybar_service::ops::{self, ListReport, PackRequest};
use prybar_service::Result;
use std::fmt::Write;

pub fn list(a: &ListArgs, out: &Output) -> Result<u8> {
    let report = ops::list(&a.package, a.filter.as_deref(), a.limit)?;
    out.emit(&report, text);
    Ok(0)
}

pub fn unpack(a: &UnpackArgs, out: &Output) -> Result<u8> {
    out.progress(&format!("unpacking {}", a.package.display()));
    let report = ops::unpack(&a.package, &a.out)?;
    out.emit(&report, pretty);
    Ok(0)
}

pub fn pack(a: &PackArgs, out: &Output) -> Result<u8> {
    out.progress(&format!("packing {}", a.src.display()));
    let report = ops::pack(&PackRequest {
        src: a.src.clone(),
        out: a.out.clone(),
        vpk_version: a.vpk_version,
        chunk_size: a.chunk_size,
    })?;
    out.emit(&report, pretty);
    Ok(0)
}

fn text(r: &ListReport) -> String {
    let mut s = String::new();
    for e in &r.entries {
        let _ = writeln!(s, "{:>12}  {}", e.size, e.path);
    }
    let more = if r.truncated { " (truncated)" } else { "" };
    let _ = writeln!(s, "{} files, {} bytes{more}", r.total, r.total_bytes);
    s
}
