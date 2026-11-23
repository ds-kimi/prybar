use prybar_workshop::publish;
use prybar_workshop::spec::{stage_content, ItemSpec, GMOD_APP_ID};

use super::common::gmod_spec;

#[test]
fn valid_create_passes_offline() {
    let tmp = tempfile::tempdir().unwrap();
    let report = publish(&gmod_spec(tmp.path()), true, &mut |_| {}).unwrap();
    assert!(report.dry_run && report.created && report.item_id.is_none());
    let staged = stage_content(GMOD_APP_ID, &tmp.path().join("addon.gma")).unwrap();
    assert!(staged.unwrap().path().join("addon.gma").is_file());
}

#[test]
fn update_needs_only_an_id_and_non_gmod_takes_a_folder() {
    let tmp = tempfile::tempdir().unwrap();
    let update = ItemSpec {
        app_id: GMOD_APP_ID,
        item_id: Some(42),
        change_note: Some("fix".into()),
        ..ItemSpec::default()
    };
    let report = publish(&update, true, &mut |_| {}).unwrap();
    assert_eq!(report.item_id, Some(42));
    std::fs::create_dir_all(tmp.path().join("maps")).unwrap();
    std::fs::write(tmp.path().join("maps/ctf_test.bsp"), b"VBSP").unwrap();
    let tf2 = ItemSpec {
        app_id: 440,
        title: Some("hat".into()),
        content: Some(tmp.path().to_path_buf()),
        ..ItemSpec::default()
    };
    assert!(publish(&tf2, true, &mut |_| {}).is_ok());
    assert!(stage_content(440, tmp.path()).unwrap().is_none());
}

#[cfg(not(feature = "workshop"))]
#[test]
fn real_calls_without_feature_say_so() {
    let tmp = tempfile::tempdir().unwrap();
    let err = publish(&gmod_spec(tmp.path()), false, &mut |_| {}).unwrap_err();
    assert!(matches!(err, prybar_workshop::WorkshopError::Disabled));
    assert!(matches!(
        prybar_workshop::list_items(GMOD_APP_ID),
        Err(prybar_workshop::WorkshopError::Disabled)
    ));
}
