use prybar_workshop::item::{record, series};

use super::item::item;

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
