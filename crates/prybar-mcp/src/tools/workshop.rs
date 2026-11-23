use crate::params::{GameArgs, ItemArgs, ItemsArgs};
use crate::reply::blocking;
use crate::server::Prybar;
use prybar_service::tools;
use prybar_service::workshop::{self, ItemQuery};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router};

#[tool_router(router = workshop_router, vis = "pub(crate)")]
impl Prybar {
    #[tool(
        description = "Check Steam for one Workshop game before publishing: client running, user signed in (name, id), game owned. `state` names the one thing to fix. Needs a `workshop` build.",
        annotations(read_only_hint = true, open_world_hint = true)
    )]
    async fn workshop_status(&self, Parameters(a): Parameters<GameArgs>) -> CallToolResult {
        blocking(move || Ok(tools::steam_status(tools::workshop_app_id(&a.game)?))).await
    }

    #[tool(
        description = "The signed-in user's Workshop items for a game, searched, filtered and sorted like the desktop app's list. Returns `total` before filtering, `tags` in use and `items` (id, title, description in BBCode, tags, visibility, subscriptions, favorites, votes, views, comments, times). Also records today's stats point for workshop_item's chart.",
        annotations(read_only_hint = true, open_world_hint = true)
    )]
    async fn workshop_items(&self, Parameters(a): Parameters<ItemsArgs>) -> CallToolResult {
        blocking(move || {
            let q = ItemQuery {
                search: a.search,
                sort: a.sort,
                reverse: a.reverse,
                visibility: a.visibility,
                tag: a.tag,
            };
            workshop::workshop_items(tools::workshop_app_id(&a.game)?, &q)
        })
        .await
    }

    #[tool(
        description = "One of the user's Workshop items in full: current fields and stats, its page `url`, daily `stats` points (subscriptions, favorites; one per day a list was read) and version `history` (time, headline, change note) from the public changelog page, cached a day. `history_error` says why the history is missing (Steam rate limit or offline). To change the item, write a spec with its `item_id` and call publish_workshop.",
        annotations(read_only_hint = true, open_world_hint = true)
    )]
    async fn workshop_item(&self, Parameters(a): Parameters<ItemArgs>) -> CallToolResult {
        blocking(move || workshop::workshop_item(tools::workshop_app_id(&a.game)?, a.item_id)).await
    }
}
