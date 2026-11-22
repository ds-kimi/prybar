#[path = "../gmod_flex/load.rs"]
mod load;

use load::{model, stems};
use prybar_export::qc::{decompile_files, QcOptions};
use prybar_testkit::gmod_dir_vpk;
use prybar_vpk::Vpk;

/// Every GMod model decompiles to a QC whose file references all exist
/// and whose braces balance; compiling it back is checked by the round-trip harness.
#[test]
fn writes_qc_for_real_models() {
    let Some(vpk) = gmod_dir_vpk().map(|p| Vpk::open(&p).unwrap()) else {
        return eprintln!("skipped: garrysmod_dir.vpk not found");
    };
    let (mut done, mut sequences, mut flexed) = (0, 0, 0);
    for stem in stems(&vpk, ".mdl") {
        let Some(ir) = model(&vpk, stem) else {
            continue;
        };
        let files = decompile_files(&ir, QcOptions::default());
        let qc = &files.last().unwrap().text;
        let written: Vec<&str> = files.iter().map(|f| f.name.as_str()).collect();
        for token in qc.split('"').skip(1).step_by(2) {
            if token.ends_with(".smd") || token.ends_with(".vta") || token.ends_with(".vrd") {
                assert!(written.contains(&token), "{stem}: {token}");
            }
        }
        assert_eq!(qc.matches('{').count(), qc.matches('}').count(), "{stem}");
        sequences += qc.matches("$sequence ").count();
        flexed += usize::from(qc.contains("flexfile"));
        done += 1;
    }
    eprintln!("{done} QCs, {sequences} sequences, {flexed} flexed");
    assert!(done > 1000 && sequences > 0 && flexed > 0);
}
