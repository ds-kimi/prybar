#![no_main]

use libfuzzer_sys::fuzz_target;
use prybar_export::qc::{decompile_files, QcOptions};

// Parsed headers are hostile IR: every index the QC writer follows comes
// straight from the input.
fuzz_target!(|data: &[u8]| {
    if let Ok(ir) = prybar_mdl::parse(data) {
        let _ = decompile_files(&ir, QcOptions { split: true });
    }
});
