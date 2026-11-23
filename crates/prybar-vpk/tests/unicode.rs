//! Community content often has non-English file names; a Crowbar fork had
//! to fix a crash on them, so prybar pins the behavior.

use prybar_vpk::{extract_all, pack, PackOptions, Vpk};

#[test]
fn non_ascii_names_pack_list_and_extract() {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    let names = [
        "materials/café/größe.vmt",
        "sound/музыка/трек.wav",
        "models/箱子.mdl",
    ];
    for (i, rel) in names.iter().enumerate() {
        let p = src.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, [i as u8; 3]).unwrap();
    }
    let dir = tmp.path().join("u.vpk");
    let opts = PackOptions {
        version: 2,
        chunk_size: None,
    };
    assert_eq!(pack(&src, &dir, &opts).unwrap(), 3);
    let vpk = Vpk::open(&dir).unwrap();
    for rel in names {
        assert!(vpk.entries.iter().any(|e| e.path == rel), "{rel} missing");
    }
    let out = tmp.path().join("out");
    assert_eq!(extract_all(&vpk, &out).unwrap(), 3);
    assert_eq!(std::fs::read(out.join(names[1])).unwrap(), [1u8; 3]);
}
