use prybar_workshop::spec::ItemSpec;

use super::common::{field, gmod_spec};

#[test]
fn hostile_inputs_are_rejected_by_field() {
    let tmp = tempfile::tempdir().unwrap();
    let base = gmod_spec(tmp.path());
    let with = |f: &dyn Fn(&mut ItemSpec)| {
        let mut s = base.clone();
        f(&mut s);
        field(&s)
    };
    assert_eq!(with(&|s| s.app_id = 0), "app_id");
    assert_eq!(with(&|s| s.title = Some("a\0b".into())), "title");
    assert_eq!(with(&|s| s.title = Some("x".repeat(129))), "title");
    assert_eq!(with(&|s| s.title = None), "title");
    assert_eq!(
        with(&|s| s.description = Some("d".repeat(8000))),
        "description"
    );
    assert_eq!(with(&|s| s.tags = vec!["a,b".into()]), "tags");
    assert_eq!(with(&|s| s.tags = vec!["t".into(); 33]), "tags");
    assert_eq!(
        with(&|s| s.preview = Some(tmp.path().join("p.bmp"))),
        "preview"
    );
    assert_eq!(
        with(&|s| s.content = Some(tmp.path().to_path_buf())),
        "content"
    );
}
