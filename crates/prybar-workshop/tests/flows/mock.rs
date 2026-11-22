use prybar_workshop::mock::MockSteam;
use prybar_workshop::spec::{ItemSpec, GMOD_APP_ID};
use prybar_workshop::{publish_with, Steam, SteamStatus};

use super::fixture::{addon, png, GOOD};

fn spec(dir: &std::path::Path) -> ItemSpec {
    ItemSpec {
        app_id: GMOD_APP_ID,
        title: Some("Hello".into()),
        content: Some(addon(dir, GOOD, &[])),
        preview: Some(png(dir, 300, 200)),
        ..Default::default()
    }
}

#[test]
fn new_item_from_folder_is_packed_tagged_and_resized() {
    let tmp = tempfile::tempdir().unwrap();
    let steam = MockSteam::ready();
    let mut stages = Vec::new();
    let r = publish_with(&steam, &spec(tmp.path()), false, &mut |p| {
        stages.push(p.stage.clone())
    });
    let r = r.unwrap();
    assert!(r.created && r.item_id.is_some() && stages.len() == 3);
    let sent = steam.uploads.lock().unwrap()[0].clone();
    assert_eq!(sent.tags, ["Addon", "tool", "fun", "build"]);
    // The folder went out packed into a staged copy, not as is.
    assert_ne!(sent.content, spec(tmp.path()).content);
    assert!(sent.preview.unwrap().ends_with("preview.jpg"));
}

#[test]
fn update_keeps_id_and_reports_legal_and_failures() {
    let tmp = tempfile::tempdir().unwrap();
    let mut steam = MockSteam::ready();
    steam.needs_legal_agreement = true;
    let mut s = spec(tmp.path());
    s.item_id = Some(42);
    let r = publish_with(&steam, &s, false, &mut |_| {}).unwrap();
    assert_eq!(
        (r.item_id, r.created, r.needs_legal_agreement),
        (Some(42), false, true)
    );
    steam.fail = Some("limit exceeded".into());
    let err = publish_with(&steam, &s, false, &mut |_| {}).unwrap_err();
    assert!(err.to_string().contains("limit exceeded"));
}

#[test]
fn dry_run_never_reaches_steam_and_status_is_scripted() {
    let tmp = tempfile::tempdir().unwrap();
    let mut steam = MockSteam::ready();
    publish_with(&steam, &spec(tmp.path()), true, &mut |_| {}).unwrap();
    assert!(steam.uploads.lock().unwrap().is_empty());
    steam.status = SteamStatus::NotOwned;
    assert_eq!(steam.status(GMOD_APP_ID), SteamStatus::NotOwned);
    assert!(steam.list(GMOD_APP_ID).is_err());
}
