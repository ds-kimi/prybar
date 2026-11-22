#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = prybar_mdl::inspect(data);
    let _ = prybar_mdl::parse(data);
    let _ = prybar_mdl::embedded(data);
});
