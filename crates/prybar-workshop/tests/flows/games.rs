use prybar_workshop::games::{by_app, by_id, unsupported, Content, WORKSHOP_GAMES};
use prybar_workshop::mock::MockSteam;
use prybar_workshop::spec::ItemSpec;
use prybar_workshop::{publish_with, WorkshopError};

use super::fixture::{addon, GOOD};

/// Content that passes each game's rules, built in `dir`.
fn content(dir: &std::path::Path, kind: Content) -> std::path::PathBuf {
    match kind {
        Content::Gma => addon(dir, GOOD, &[]),
        Content::Maps => {
            let maps = dir.join("map").join("maps");
            std::fs::create_dir_all(&maps).unwrap();
            std::fs::write(maps.join("ctf_crate.bsp"), b"VBSP").unwrap();
            dir.join("map")
        }
    }
}

#[test]
fn every_supported_game_publishes_on_mock_steam() {
    for game in WORKSHOP_GAMES {
        let tmp = tempfile::tempdir().unwrap();
        let steam = MockSteam::ready();
        let spec = ItemSpec {
            app_id: game.app_id,
            title: Some(format!("On {}", game.name)),
            content: Some(content(tmp.path(), game.content)),
            ..Default::default()
        };
        let r = publish_with(&steam, &spec, false, &mut |_| {}).unwrap();
        assert!(r.created, "{}", game.id);
        assert_eq!(steam.uploads.lock().unwrap()[0].app_id, game.app_id);
    }
}

#[test]
fn games_resolve_by_name_or_app_id_and_others_say_why() {
    assert_eq!(by_id("GarrysMod").map(|g| g.app_id), Some(4000));
    assert_eq!(by_id(" 440 ").map(|g| g.id), Some("tf2"));
    assert!(by_app(550).is_none() && by_id("l4d2").is_none());
    assert!(unsupported("550").is_some_and(|u| u.reason.contains("legacy")));
    assert!(by_id("").is_none() && by_id("99999999999999").is_none());
}

#[test]
fn wrong_game_or_content_is_refused_before_upload() {
    let tmp = tempfile::tempdir().unwrap();
    let steam = MockSteam::ready();
    let field = |app_id, dir: &std::path::Path| {
        let spec = ItemSpec {
            app_id,
            title: Some("x".into()),
            content: Some(dir.to_path_buf()),
            ..Default::default()
        };
        match publish_with(&steam, &spec, false, &mut |_| {}) {
            Err(WorkshopError::Invalid { field, .. }) => field,
            other => panic!("{other:?}"),
        }
    };
    assert_eq!(field(550, tmp.path()), "app_id");
    assert_eq!(field(440, tmp.path()), "content");
    assert!(steam.uploads.lock().unwrap().is_empty());
}
