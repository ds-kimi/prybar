#![no_main]

use libfuzzer_sys::fuzz_target;

// The first two bytes give the MDL length; the rest is the `.ani` file.
fuzz_target!(|data: &[u8]| {
    let Some((len, rest)) = data.split_first_chunk::<2>() else {
        return;
    };
    let cut = usize::from(u16::from_le_bytes(*len)).min(rest.len());
    let (mdl, ani) = rest.split_at(cut);
    let _ = prybar_mdl::parse_with_ani(mdl, Some(ani));
});
