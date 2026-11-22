use super::fixture::{self, put, ANIM, DATA};
use proptest::prelude::*;

#[test]
fn looping_chain_stops() {
    let mut mdl = fixture::build(false);
    // Two empty records pointing at each other.
    put(&mut mdl, DATA, &[0, 0, 4, 0, 1, 0, 0xfc, 0xff]);
    let ir = prybar_mdl::parse(&mdl).unwrap();
    assert_eq!(ir.animations[0].frames.len(), 3);
}

#[test]
fn huge_frame_count_is_refused() {
    let mut mdl = fixture::build(false);
    put(&mut mdl, ANIM + 16, &i32::MAX.to_le_bytes());
    assert!(prybar_mdl::parse(&mdl).is_err());
    put(&mut mdl, ANIM + 16, &(-1i32).to_le_bytes());
    assert!(prybar_mdl::parse(&mdl).is_err());
}

#[test]
fn section_past_the_end_is_an_error() {
    let mut mdl = fixture::build(false);
    put(&mut mdl, ANIM + 80, &(i32::MAX - 8).to_le_bytes());
    put(&mut mdl, ANIM + 84, &1i32.to_le_bytes());
    assert!(prybar_mdl::parse(&mdl).is_err());
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    #[test]
    fn mutated_anims_never_panic(flips in prop::collection::vec((0usize..fixture::SIZE, any::<u8>()), 0..16), ani in any::<bool>()) {
        let mut mdl = fixture::build(ani);
        let mut extra = fixture::ani();
        for &(at, v) in &flips {
            mdl[at] = v;
            let n = extra.len();
            extra[at % n] = v;
        }
        let _ = prybar_mdl::parse_with_ani(&mdl, Some(&extra));
    }
}
