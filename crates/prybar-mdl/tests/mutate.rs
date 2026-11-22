//! Random truncations and byte flips of a valid model must give errors,
//! never panics or runaway allocations.

mod common;

use proptest::prelude::*;

fn mutate(mut data: Vec<u8>, flips: &[(usize, u8)], cut: usize) -> Vec<u8> {
    for &(at, v) in flips {
        let i = at % data.len();
        data[i] = v;
    }
    data.truncate(cut % (data.len() + 1));
    data
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    #[test]
    fn mutated_inputs_never_panic(flips in prop::collection::vec(any::<(usize, u8)>(), 0..16), cut: usize) {
        let mdl = mutate(common::build(), &flips, cut);
        let _ = prybar_mdl::inspect(&mdl);
        if let Ok(mut ir) = prybar_mdl::parse(&mdl) {
            let vvd = prybar_mdl::parse_vvd(&mutate(common::geometry::vvd(), &flips, cut));
            let vtx = prybar_mdl::parse_vtx(&mutate(common::geometry::vtx(), &flips, cut));
            if let (Ok(vvd), Ok(vtx)) = (vvd, vtx) {
                let _ = prybar_mdl::load_geometry(&mut ir, &vvd, &vtx);
            }
        }
    }
}
