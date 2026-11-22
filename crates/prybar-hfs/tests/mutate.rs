//! Random truncations and byte flips of a valid package must give errors,
//! never panics.

mod support;

use proptest::prelude::*;
use prybar_hfs::Hfs;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    #[test]
    fn mutated_inputs_never_panic(flips in prop::collection::vec(any::<(usize, u8)>(), 0..16), cut: usize) {
        let mut data = support::build(&[("a.txt", b"hello"), ("d/b.bin", b"\x01\x02")]);
        for (at, v) in flips {
            let i = at % data.len();
            data[i] = v;
        }
        data.truncate(cut % (data.len() + 1));
        if let Ok(hfs) = Hfs::from_bytes(data) {
            for e in &hfs.entries {
                let _ = hfs.read(e);
            }
        }
    }
}
