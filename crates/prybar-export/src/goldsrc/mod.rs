//! GoldSrc-only outputs: the embedded textures, written back as the
//! 8-bit BMPs studiomdl compiled them from.

mod bmp;

use crate::smd::names::stem;
pub use bmp::bmp;
use prybar_mdl::ModelIr;
use std::collections::HashSet;

/// One BMP per embedded texture with a sane size; names stay inside the
/// output folder and never collide (case-insensitively).
pub fn texture_files(ir: &ModelIr) -> Vec<(String, Vec<u8>)> {
    let mut taken = HashSet::new();
    ir.textures
        .iter()
        .filter_map(|t| {
            let data = bmp(t)?;
            let raw = stem(&t.name, "texture");
            // Byte-wise check: lowercasing could shift char boundaries.
            let cut = raw.len().saturating_sub(4);
            let has_ext = raw
                .get(cut..)
                .is_some_and(|e| e.eq_ignore_ascii_case(".bmp"));
            let base = match if has_ext { &raw[..cut] } else { &raw[..] } {
                "" => "texture".to_owned(),
                b => b.to_owned(),
            };
            let mut name = format!("{base}.bmp");
            let mut n = 2;
            while !taken.insert(name.to_lowercase()) {
                name = format!("{base}_{n}.bmp");
                n += 1;
            }
            Some((name, data))
        })
        .collect()
}
