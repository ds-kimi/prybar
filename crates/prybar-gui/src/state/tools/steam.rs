//! What the publish screen knows about Steam: is it ready, which items
//! the user already published, and the check of the chosen addon folder.

use std::path::PathBuf;
use std::sync::Arc;

use gpui::{Context, RenderImage};
use prybar_workshop::addon::AddonCheck;
use prybar_workshop::{ListedItem, SteamStatus};

use crate::state::AppState;

#[derive(Default)]
pub struct SteamView {
    /// `None` while the check runs.
    pub status: Option<SteamStatus>,
    pub items: Vec<ListedItem>,
    /// Why the list could not be read, when Steam was ready.
    pub items_error: Option<String>,
    /// When `items` came from Steam, for the "read 5 min ago" tips.
    pub items_read: Option<std::time::SystemTime>,
    /// The folder the check is about, and what it found.
    pub addon: Option<(PathBuf, Result<AddonCheck, String>)>,
    /// The account picture, read once and kept: it rarely changes and
    /// each read wakes the Steam API.
    pub avatar: Option<Arc<RenderImage>>,
    pub catalog: super::Catalog,
}

impl SteamView {
    /// What the state means for the user, what to do about it, and
    /// whether it is good news. `None` while the check runs.
    pub fn sentence(&self) -> (String, Option<bool>) {
        let bad = |text: &str| (text.to_owned(), Some(false));
        match &self.status {
            None => ("Checking Steam…".to_owned(), None),
            Some(SteamStatus::Ready { user }) => (
                format!("Steam is ready, logged in as {user}. Publishing works."),
                Some(true),
            ),
            Some(SteamStatus::NotRunning) => {
                bad("Steam is not running. Start Steam, then click to check again.")
            }
            Some(SteamStatus::NotLoggedIn) => {
                bad("Steam is open but no one is logged in. Log in, then click to check again.")
            }
            Some(SteamStatus::NotOwned) => {
                bad("This Steam account does not own Garry's Mod, so it cannot publish for it.")
            }
            Some(SteamStatus::Disabled) => {
                bad("This build has no Steam Workshop support. Dry run still checks everything.")
            }
        }
    }
}

impl AppState {
    /// Read-only Steam calls off the UI thread; Retry calls it again.
    pub fn refresh_steam(&mut self, cx: &mut Context<Self>) {
        if self.tools.steam.status.is_none() && self.tools.steam_started {
            return;
        }
        self.tools.steam_started = true;
        self.tools.steam.status = None;
        let b = self.tools.backend;
        let want_avatar = self.tools.steam.avatar.is_none();
        let work = cx
            .background_executor()
            .spawn(async move { super::ask::ask(b, want_avatar) });
        cx.spawn(async move |this, cx| {
            let (s, list, face) = work.await;
            let _ = this.update(cx, |st, cx| {
                st.tools.steam.status = Some(s);
                let steam = &mut st.tools.steam;
                steam.avatar = steam.avatar.take().or(face);
                steam.items_read = Some(std::time::SystemTime::now());
                (steam.items, steam.items_error) = match list {
                    Ok(items) => (items, None),
                    Err(why) => (Vec::new(), Some(why)),
                };
                let items = st.tools.steam.items.clone();
                st.record_stats(&items, cx);
                cx.notify();
            });
        })
        .detach();
        cx.notify();
    }

    /// Checks an addon folder when the content path names one; a `.gma`
    /// or an empty path clears the panel.
    pub fn check_content(&mut self, path: &str, cx: &mut Context<Self>) {
        let path = PathBuf::from(path.trim());
        // Called on every redraw of the field, so stay quiet when nothing changes.
        if !path.is_dir() {
            if self.tools.steam.addon.take().is_some() {
                cx.notify();
            }
            return;
        }
        if self.tools.steam.addon.as_ref().is_some_and(|a| a.0 == path) {
            return;
        }
        let check = self.tools.backend.addon;
        let dir = path.clone();
        let work = cx.background_executor().spawn(async move { check(&dir) });
        cx.spawn(async move |this, cx| {
            let found = work.await;
            let _ = this.update(cx, |st, cx| {
                st.tools.steam.addon = Some((path, found));
                cx.notify();
            });
        })
        .detach();
    }
}
