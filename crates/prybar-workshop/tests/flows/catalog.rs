use prybar_workshop::catalog::{apply, rating, tags, Query, Sort};
use prybar_workshop::mock::MockSteam;
use prybar_workshop::spec::{Visibility, GMOD_APP_ID};
use prybar_workshop::{ListedItem, Steam};

/// 250 items with stats that differ per sort, so each order is checkable.
fn listed() -> Vec<ListedItem> {
    let mut steam = MockSteam::ready();
    steam.items = (0..250u32)
        .map(|i| ListedItem {
            item_id: u64::from(1000 + i),
            title: format!("Item {:03}", 249 - i),
            tags: vec![
                "Addon".into(),
                if i % 2 == 0 { "fun" } else { "build" }.into(),
            ],
            time_updated: 1_000 + i,
            time_created: 5_000 - i,
            visibility: if i % 5 == 0 {
                Visibility::Private
            } else {
                Visibility::Public
            },
            subscriptions: u64::from(i * 7 % 250),
            favorites: u64::from(i % 13),
            votes_up: i,
            votes_down: 250 - i,
            preview_url: None,
        })
        .collect();
    steam.list(GMOD_APP_ID).unwrap()
}

fn key(it: &ListedItem, sort: Sort) -> u64 {
    match sort {
        Sort::Subscribed => it.subscriptions,
        Sort::Rated => u64::from(rating(it)),
        Sort::Favorited => it.favorites,
        Sort::Updated => u64::from(it.time_updated),
        Sort::Newest => u64::from(it.time_created),
        Sort::Title => it.title[5..].parse().unwrap(),
    }
}

#[test]
fn every_sort_orders_both_ways() {
    let items = listed();
    for sort in Sort::ALL {
        for descending in [false, true] {
            let q = Query {
                sort,
                descending,
                ..Query::default()
            };
            let order = apply(&items, &q);
            assert_eq!(order.len(), 250);
            let key = |i: usize| key(&items[i], sort);
            let ok = order.windows(2).all(|w| {
                if descending {
                    key(w[0]) >= key(w[1])
                } else {
                    key(w[0]) <= key(w[1])
                }
            });
            assert!(ok, "{sort:?} descending={descending}");
        }
    }
}

#[test]
fn search_and_filters_combine() {
    let items = listed();
    let q = |text: &str| Query {
        text: text.into(),
        ..Query::default()
    };
    let find = |text: &str| apply(&items, &q(text));
    assert_eq!(find("item 007").len(), 1);
    assert_eq!(find("FUN").len(), 125);
    assert_eq!(find("1249"), vec![249]);
    assert!(find("nothing like it").is_empty());
    assert_eq!(find("  ").len(), 250);
    let q = Query {
        visibility: Some(Visibility::Private),
        tag: Some("FUN".into()),
        ..Query::default()
    };
    // Private is every 5th, fun every 2nd: every 10th survives.
    assert_eq!(apply(&items, &q).len(), 25);
    assert_eq!(tags(&items), ["Addon", "build", "fun"]);
    let mut it = items[0].clone();
    (it.votes_up, it.votes_down) = (0, 0);
    assert_eq!(rating(&it), 500, "no votes is neutral");
}
