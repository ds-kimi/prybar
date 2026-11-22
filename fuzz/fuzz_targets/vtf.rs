#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(vtf) = prybar_vtf::Vtf::parse(data.to_vec()) {
        for level in 0..vtf.header.mips.min(16) {
            let _ = vtf.decode(level);
        }
    }
});
