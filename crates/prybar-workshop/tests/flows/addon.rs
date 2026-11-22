use prybar_workshop::addon::check_addon;

use super::fixture::{addon, GOOD};

#[test]
fn good_folder_passes() {
    let tmp = tempfile::tempdir().unwrap();
    let check = check_addon(&addon(tmp.path(), GOOD, &[])).unwrap();
    assert!(check.ok(), "{check:?}");
    assert_eq!((check.files, check.kind.as_str()), (1, "tool"));
}

#[test]
fn rejected_files_are_listed_with_a_reason() {
    let tmp = tempfile::tempdir().unwrap();
    let extra = [
        "notes.txt",
        "models/a.sw.vtx",
        "lua/../x.exe",
        "models/a.mdl",
    ];
    let check = check_addon(&addon(tmp.path(), GOOD, &extra)).unwrap();
    let paths: Vec<&str> = check.rejected.iter().map(|r| r.path.as_str()).collect();
    assert!(paths.contains(&"notes.txt") && paths.contains(&"models/a.sw.vtx"));
    assert!(!paths.contains(&"models/a.mdl"));
    assert!(check.rejected.iter().all(|r| !r.reason.is_empty()));
    assert!(!check.ok());
}

#[test]
fn hostile_addon_json_reports_every_problem() {
    let tmp = tempfile::tempdir().unwrap();
    let json = r#"{"title":" ","type":"virus","tags":["fun","nope","build"]}"#;
    let check = check_addon(&addon(tmp.path(), json, &[])).unwrap();
    assert_eq!(check.problems.len(), 4, "{:?}", check.problems);
    for bad in ["{", "[]", "{\"title\":1}", "\u{0}"] {
        let dir = tempfile::tempdir().unwrap();
        assert!(check_addon(&addon(dir.path(), bad, &[])).is_err());
    }
    assert!(check_addon(tmp.path()).is_err(), "no addon.json");
}

#[test]
fn folder_is_checked_by_dry_run() {
    let tmp = tempfile::tempdir().unwrap();
    let spec = prybar_workshop::spec::ItemSpec {
        app_id: prybar_workshop::spec::GMOD_APP_ID,
        title: Some("t".into()),
        content: Some(addon(tmp.path(), GOOD, &["bad.exe"])),
        ..Default::default()
    };
    let err = prybar_workshop::publish(&spec, true, &mut |_| {}).unwrap_err();
    assert!(err.to_string().contains("bad.exe"), "{err}");
}
