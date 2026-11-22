use std::sync::mpsc::channel;

use steamworks::{
    AppIDs, AppId, PublishedFileVisibility as V, QueryResults, UGCStatisticType, UGCType, UserList,
    UserListOrder,
};

use super::wait::{init, wait};
use crate::spec::Visibility;
use crate::{ListedItem, WorkshopError};

pub fn list(app_id: u32) -> Result<Vec<ListedItem>, WorkshopError> {
    let client = init(app_id)?;
    let app = AppId(app_id);
    let account = client.user().steam_id().account_id();
    let ids = AppIDs::Both {
        creator: app,
        consumer: app,
    };
    let order = UserListOrder::LastUpdatedDesc;
    let query = client
        .ugc()
        .query_user(account, UserList::Published, UGCType::Items, order, ids, 1)
        .map_err(|e| WorkshopError::Steam(e.to_string()))?;
    let (tx, rx) = channel();
    // QueryResults borrows Steam memory, so copy out inside the callback.
    query.fetch(move |res| {
        let items = res.map(|r| listed(&r));
        let _ = tx.send(items);
    });
    wait(&client, &rx, || {})?.map_err(|e| WorkshopError::Steam(e.to_string()))
}

fn listed(r: &QueryResults) -> Vec<ListedItem> {
    let rows = r
        .iter()
        .enumerate()
        .filter_map(|(i, q)| Some((i as u32, q?)));
    rows.map(|(i, q)| ListedItem {
        item_id: q.published_file_id.0,
        title: q.title,
        tags: q.tags,
        time_updated: q.time_updated,
        visibility: match q.visibility {
            V::Public => Visibility::Public,
            V::FriendsOnly => Visibility::FriendsOnly,
            V::Unlisted => Visibility::Unlisted,
            V::Private => Visibility::Private,
        },
        subscriptions: r.statistic(i, UGCStatisticType::Subscriptions).unwrap_or(0),
        preview_url: r.preview_url(i),
    })
    .collect()
}
