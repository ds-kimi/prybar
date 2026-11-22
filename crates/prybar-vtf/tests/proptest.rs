mod support;

use proptest::prelude::*;
use prybar_vtf::Vtf;

proptest! {
    #[test]
    fn random_bytes_never_panic(bytes in proptest::collection::vec(any::<u8>(), 0..512)) {
        if let Ok(v) = Vtf::parse(bytes) {
            for m in 0..v.header.mips.min(4) {
                let _ = v.decode(m);
            }
        }
    }

    #[test]
    fn valid_images_decode_to_full_size(w in 1u16..64, h in 1u16..64, id in prop::sample::select(vec![0, 2, 3, 5, 6, 8, 12, 13, 14, 15])) {
        let f = prybar_vtf::Format::from_id(id).unwrap();
        let data = vec![0x33; f.size(w as u32, h as u32).unwrap()];
        let img = Vtf::parse(support::vtf(5, id, (w, h), 1, &data)).unwrap().decode(0).unwrap();
        prop_assert_eq!(img.rgba.len(), w as usize * h as usize * 4);
    }
}
