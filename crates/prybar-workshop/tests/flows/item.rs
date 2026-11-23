use prybar_workshop::item::{changes, record, series, Draft, Links};
use prybar_workshop::mock::MockSteam;
use prybar_workshop::spec::{Visibility, GMOD_APP_ID};
use prybar_workshop::{publish_with, ListedItem};

use super::fixture::png;

fn item() -> ListedItem {
    ListedItem {
        item_id: 42,
        title: "Crates".into(),
        description: "Old text".into(),
        tags: vec!["Addon".into(), "fun".into()],
        visibility: Visibility::Public,
        subscriptions: 10,
        favorites: 2,
        ..Default::default()
    }
}

#[test]
fn an_untouched_draft_sends_nothing_but_the_id() {
    let spec = changes(GMOD_APP_ID, &item(), &Draft::from_item(&item()));
    assert_eq!(spec.item_id, Some(42));
    assert!(spec.title.is_none() && spec.description.is_none() && spec.visibility.is_none());
    assert!(spec.tags.is_empty() && spec.preview.is_none() && spec.content.is_none());
}

#[test]
fn only_changed_fields_reach_steam_and_the_image_is_replaced() {
    let tmp = tempfile::tempdir().unwrap();
    let mut d = Draft::from_item(&item());
    d.title = "Crates v2 ".into();
    d.tags = vec!["FUN".into(), "addon".into()];
    d.preview = Some(png(tmp.path(), 640, 480));
    d.change_note = "Bigger crates".into();
    let steam = MockSteam::ready();
    let spec = changes(GMOD_APP_ID, &item(), &d);
    publish_with(&steam, &spec, false, &mut |_| {}).unwrap();
    let sent = steam.uploads.lock().unwrap()[0].clone();
    assert_eq!(sent.title.as_deref(), Some("Crates v2"));
    assert!(sent.description.is_none() && sent.visibility.is_none() && sent.tags.is_empty());
    assert!(sent.preview.unwrap().ends_with("preview.jpg"));
    assert_eq!(sent.change_note.as_deref(), Some("Bigger crates"));
}

#[test]
fn folder_links_survive_a_round_trip_and_damage() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join("sub/links.txt");
    let mut links = Links::default();
    links.0.insert(42, "/addons/crates".into());
    links.0.insert(7, "/bad\npath".into());
    links.save(&file).unwrap();
    assert_eq!(Links::load(&file).0.len(), 1);
    std::fs::write(&file, "x\ty\n\n42\n9\t/ok\n").unwrap();
    assert_eq!(Links::load(&file).0.get(&9).unwrap().to_str(), Some("/ok"));
    assert!(Links::load(&tmp.path().join("none")).0.is_empty());
}

#[test]
fn snapshots_keep_one_row_per_day_and_skip_garbage() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join("stats.tsv");
    let mut it = item();
    record(&file, &[it.clone()], 100).unwrap();
    it.subscriptions = 15;
    record(&file, &[it.clone()], 100).unwrap();
    it.subscriptions = 20;
    record(&file, &[it], 101).unwrap();
    let mut text = std::fs::read_to_string(&file).unwrap();
    text.push_str("garbage\n42\t-1\t3\t4\n42\t99999999999\t1\t1\n");
    std::fs::write(&file, text).unwrap();
    let pts = series(&file, 42);
    let subs: Vec<u64> = pts.iter().map(|p| p.subscriptions).collect();
    assert_eq!(subs, [15, 20]);
    assert!(series(&file, 1).is_empty());
}
