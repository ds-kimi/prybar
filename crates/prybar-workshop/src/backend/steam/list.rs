use std::sync::mpsc::channel;

use steamworks::{
    AppIDs, AppId, Client, PublishedFileVisibility as V, QueryResults, UGCStatisticType, UGCType,
    UserList, UserListOrder,
};

use super::wait::{init, wait};
use crate::spec::Visibility;
use crate::{ListedItem, WorkshopError};

/// Steam pages hold 50; this caps a runaway answer at 2000 items.
const MAX_PAGES: u32 = 40;

/// Every page of the user's items, newest update first.
pub fn list(app_id: u32) -> Result<Vec<ListedItem>, WorkshopError> {
    let client = init(app_id)?;
    let mut all = Vec::new();
    for n in 1..=MAX_PAGES {
        let (items, total) = page(&client, app_id, n)?;
        let short = items.is_empty();
        all.extend(items);
        if short || all.len() as u32 >= total {
            break;
        }
    }
    Ok(all)
}

fn page(client: &Client, app_id: u32, n: u32) -> Result<(Vec<ListedItem>, u32), WorkshopError> {
    let app = AppId(app_id);
    let account = client.user().steam_id().account_id();
    let ids = AppIDs::Both {
        creator: app,
        consumer: app,
    };
    let order = UserListOrder::LastUpdatedDesc;
    let query = client
        .ugc()
        .query_user(account, UserList::Published, UGCType::Items, order, ids, n)
        .map_err(|e| WorkshopError::Steam(e.to_string()))?
        // The edit popup starts from the full text, not the 255-char cut.
        .set_return_long_description(true);
    let (tx, rx) = channel();
    // QueryResults borrows Steam memory, so copy out inside the callback.
    query.fetch(move |res| {
        let items = res.map(|r| (listed(&r), r.total_results()));
        let _ = tx.send(items);
    });
    wait(client, &rx, || {})?.map_err(|e| WorkshopError::Steam(e.to_string()))
}

fn listed(r: &QueryResults) -> Vec<ListedItem> {
    let rows = r
        .iter()
        .enumerate()
        .filter_map(|(i, q)| Some((i as u32, q?)));
    let stat = |i, s| r.statistic(i, s).unwrap_or(0);
    rows.map(|(i, q)| ListedItem {
        item_id: q.published_file_id.0,
        title: q.title,
        description: q.description,
        tags: q.tags,
        time_updated: q.time_updated,
        time_created: q.time_created,
        visibility: match q.visibility {
            V::Public => Visibility::Public,
            V::FriendsOnly => Visibility::FriendsOnly,
            V::Unlisted => Visibility::Unlisted,
            V::Private => Visibility::Private,
        },
        subscriptions: stat(i, UGCStatisticType::Subscriptions),
        favorites: stat(i, UGCStatisticType::Favorites),
        votes_up: q.num_upvotes,
        votes_down: q.num_downvotes,
        preview_url: r.preview_url(i),
        unique_subscriptions: stat(i, UGCStatisticType::UniqueSubscriptions),
        views: stat(i, UGCStatisticType::UniqueWebsiteViews),
        comments: stat(i, UGCStatisticType::Comments),
    })
    .collect()
}
