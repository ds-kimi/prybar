//! What the publish screen knows about Steam: is it ready, which items
//! the user already published, and the check of the chosen addon folder.

use std::path::PathBuf;

use gpui::Context;
use prybar_workshop::addon::AddonCheck;
use prybar_workshop::spec::GMOD_APP_ID;
use prybar_workshop::{ListedItem, SteamStatus};

use crate::state::AppState;

#[derive(Default)]
pub struct SteamView {
    /// `None` while the check runs.
    pub status: Option<SteamStatus>,
    pub items: Vec<ListedItem>,
    /// Why the list could not be read, when Steam was ready.
    pub items_error: Option<String>,
    /// The folder the check is about, and what it found.
    pub addon: Option<(PathBuf, Result<AddonCheck, String>)>,
}

impl AppState {
    /// Read-only Steam calls off the UI thread; Retry calls it again.
    pub fn refresh_steam(&mut self, cx: &mut Context<Self>) {
        if self.tools.steam.status.is_none() && self.tools.steam_started {
            return;
        }
        self.tools.steam_started = true;
        self.tools.steam.status = None;
        let (status, items) = (self.tools.backend.status, self.tools.backend.items);
        let work = cx.background_executor().spawn(async move {
            let s = status(GMOD_APP_ID);
            let list = match s {
                SteamStatus::Ready { .. } => items(GMOD_APP_ID),
                _ => Ok(Vec::new()),
            };
            (s, list)
        });
        cx.spawn(async move |this, cx| {
            let (s, list) = work.await;
            let _ = this.update(cx, |st, cx| {
                st.tools.steam.status = Some(s);
                let steam = &mut st.tools.steam;
                (steam.items, steam.items_error) = match list {
                    Ok(items) => (items, None),
                    Err(why) => (Vec::new(), Some(why)),
                };
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
