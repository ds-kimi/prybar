use std::path::Path;

use prybar_workshop::spec::{ItemSpec, GMOD_APP_ID};
use prybar_workshop::{publish, WorkshopError};

pub fn gmod_spec(dir: &Path) -> ItemSpec {
    let gma = dir.join("addon.gma");
    std::fs::write(&gma, b"GMAD").unwrap();
    std::fs::write(dir.join("preview.jpg"), [0xff, 0xd8]).unwrap();
    ItemSpec {
        app_id: GMOD_APP_ID,
        title: Some("My addon".into()),
        tags: vec!["model".into()],
        preview: Some(dir.join("preview.jpg")),
        content: Some(gma),
        ..ItemSpec::default()
    }
}

pub fn field(spec: &ItemSpec) -> &'static str {
    match publish(spec, true, &mut |_| {}) {
        Err(WorkshopError::Invalid { field, .. }) => field,
        other => panic!("expected Invalid, got {other:?}"),
    }
}
