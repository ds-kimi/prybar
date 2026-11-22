use std::sync::mpsc::channel;

use steamworks::{AppIDs, AppId, QueryResult, UGCType, UserList, UserListOrder};

use super::wait::{init, wait};
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
        let items = res.map(|r| r.iter().flatten().map(listed).collect::<Vec<_>>());
        let _ = tx.send(items);
    });
    wait(&client, &rx, || {})?.map_err(|e| WorkshopError::Steam(e.to_string()))
}

fn listed(r: QueryResult) -> ListedItem {
    ListedItem {
        item_id: r.published_file_id.0,
        title: r.title,
        tags: r.tags,
        time_updated: r.time_updated,
    }
}
