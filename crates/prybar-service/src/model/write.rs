use crate::{Result, ServiceError};
use prybar_export::qc::{decompile_files, QcOptions};
use prybar_mdl::ModelIr;
use std::path::Path;

/// Writes every decompiled file of `ir` under `dir`; returns the count.
/// Names come from the model, so each goes through `safe_join` and a
/// name that would escape `dir` is skipped with a note.
pub fn write_model(ir: &mut ModelIr, dir: &Path, split_qc: bool) -> Result<usize> {
    let mut files: Vec<(String, Vec<u8>)> = decompile_files(ir, QcOptions { split: split_qc })
        .into_iter()
        .map(|f| (f.name, f.text.into_bytes()))
        .collect();
    files.extend(prybar_export::goldsrc::texture_files(ir));
    std::fs::create_dir_all(dir).map_err(|e| ServiceError::io(dir, e))?;
    let mut written = 0;
    for (name, bytes) in files {
        let Some(path) = prybar_io::safe_join(dir, &name) else {
            ir.notes.push(format!("skipped unsafe file name {name:?}"));
            continue;
        };
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| ServiceError::io(parent, e))?;
        }
        std::fs::write(&path, bytes).map_err(|e| ServiceError::io(&path, e))?;
        written += 1;
    }
    Ok(written)
}
