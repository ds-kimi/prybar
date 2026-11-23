//! The Workshop game the publish page shows, and switching it.

use gpui::Context;
use prybar_service::tools::WorkshopGames;
use prybar_workshop::games::{Content, WorkshopGame};

use super::ask::Answer;
use super::SteamView;
use crate::state::AppState;

impl SteamView {
    /// The game the page shows, once Steam was asked.
    pub fn target(&self) -> Option<&WorkshopGame> {
        let games = self.games.as_ref()?;
        games.detected.iter().find(|g| Some(g.app_id) == self.game)
    }

    /// Garry's Mod content (addon.json, whitelist) is checked as typed;
    /// before Steam answered the page assumes it, the usual case.
    pub fn gma(&self) -> bool {
        self.target().is_none_or(|g| g.content == Content::Gma)
    }
}

impl AppState {
    /// Switches the page to another installed game: its items replace
    /// the old ones, so the old list and selection must not linger.
    pub fn pick_game(&mut self, app_id: u32, cx: &mut Context<Self>) {
        if self.tools.steam.game == Some(app_id) || self.tools.steam.status.is_none() {
            return;
        }
        self.tools.steam.game = Some(app_id);
        self.tools.steam.items.clear();
        self.tools.steam.items_error = None;
        self.tools.item.selected = None;
        self.tools.item.series.clear();
        self.refresh_steam(cx);
    }

    /// Takes in what [`super::ask::ask`] found.
    pub(super) fn answered(
        &mut self,
        games: WorkshopGames,
        game: Option<u32>,
        a: Answer,
        cx: &mut Context<Self>,
    ) {
        let (status, list, face) = a;
        let steam = &mut self.tools.steam;
        (steam.games, steam.game, steam.status) = (Some(games), game, Some(status));
        steam.avatar = steam.avatar.take().or(face);
        steam.items_read = Some(std::time::SystemTime::now());
        (steam.items, steam.items_error) = match list {
            Ok(items) => (items, None),
            Err(why) => (Vec::new(), Some(why)),
        };
        let items = steam.items.clone();
        self.record_stats(&items, cx);
        cx.notify();
    }
}
