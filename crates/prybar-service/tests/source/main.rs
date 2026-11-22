//! Argument parsing for model references.

use prybar_service::source::ModelRef;
use std::path::PathBuf;

#[test]
fn package_refs_survive_drive_letters() {
    let r = ModelRef::parse(r"C:\games\pak_dir.VPK:models\a.mdl");
    let want = ModelRef::Packed {
        package: PathBuf::from(r"C:\games\pak_dir.VPK"),
        inner: "models/a.mdl".into(),
    };
    assert_eq!(r, want);
    assert_eq!(
        ModelRef::parse(r"C:\m\a.mdl"),
        ModelRef::Path(PathBuf::from(r"C:\m\a.mdl"))
    );
}

#[test]
fn first_package_wins_and_empty_inner_is_root() {
    match ModelRef::parse("a.gma:x.vpk:b.mdl") {
        ModelRef::Packed { package, inner } => {
            assert_eq!(package, PathBuf::from("a.gma"));
            assert_eq!(inner, "x.vpk:b.mdl");
        }
        other => panic!("{other:?}"),
    }
    assert!(
        matches!(ModelRef::parse("a.hfs:"), ModelRef::Packed { inner, .. } if inner.is_empty())
    );
}
