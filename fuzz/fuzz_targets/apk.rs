#![no_main]

use libfuzzer_sys::fuzz_target;

// Reading every entry walks all offsets the tree points at, which is
// where forged sizes and offsets would bite.
fuzz_target!(|data: &[u8]| {
    if let Ok(pkg) = prybar_apk::Apk::from_bytes(data.to_vec()) {
        for e in &pkg.entries {
            let _ = pkg.read(e);
        }
    }
});
