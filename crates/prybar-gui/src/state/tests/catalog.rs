use gpui::TestAppContext;
use prybar_workshop::catalog::Sort;

use super::tools::state;
use crate::state::tools::{prefs_lines, ItemPrefs, Thumb};

#[test]
fn item_prefs_round_trip_and_ignore_junk() {
    let p = ItemPrefs {
        sort: Sort::Rated,
        descending: false,
        grid: false,
    };
    assert_eq!(ItemPrefs::parse(&prefs_lines(p)), p);
    let junk = "items_sort=nope\nitems=x\n=\ntheme=light";
    assert_eq!(ItemPrefs::parse(junk), ItemPrefs::STEAM);
}

/// Each preview is asked once; a failure or no URL keeps the placeholder.
#[gpui::test]
fn thumbs_load_once(cx: &mut TestAppContext) {
    let state = state(cx);
    let want = vec![
        (1, Some("https://ok/a.jpg".to_owned())),
        (2, Some("https://bad".to_owned())),
        (3, None),
    ];
    state.update(cx, |s, cx| s.want_thumbs(want.clone(), cx));
    state.read_with(cx, |s, _| {
        assert_eq!(s.tools.steam.catalog.thumbs.get(&1), Some(&Thumb::Loading))
    });
    cx.run_until_parked();
    state.update(cx, |s, cx| s.want_thumbs(want, cx));
    state.read_with(cx, |s, _| {
        let t = &s.tools.steam.catalog.thumbs;
        assert_eq!(t[&1], Thumb::Ready("https://ok/a.jpg".into()));
        assert_eq!((&t[&2], &t[&3]), (&Thumb::Missing, &Thumb::Missing));
    });
}

/// A new sort lands in gui.txt; typing a search does not rewrite it.
#[gpui::test]
fn sort_is_saved_search_is_not(cx: &mut TestAppContext) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let path = tmp.path().join("gui.txt");
    let state = state(cx);
    state.update(cx, |s, cx| {
        s.prefs = Some(path.clone());
        s.set_items(|c| c.query.text = "x".into(), cx);
    });
    assert!(!path.exists(), "search alone writes nothing");
    state.update(cx, |s, cx| s.set_items(|c| c.query.sort = Sort::Title, cx));
    let saved = crate::state::prefs::load(&path).items;
    assert_eq!((saved.sort, saved.descending), (Sort::Title, true));
}
