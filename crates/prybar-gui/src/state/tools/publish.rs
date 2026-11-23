//! A Workshop upload: the form as typed, its progress, its result.

use gpui::Context;
use std::path::PathBuf;

use prybar_workshop::{Progress, PublishReport};

use super::PublishForm;
use crate::state::{AppState, ToastKind};

const NO_GAME: &str = "No supported game is installed. Install Garry's Mod or Team Fortress 2 \
                       in Steam, or add it in Settings.";

#[derive(Default)]
pub struct PublishRun {
    pub running: bool,
    pub dry_run: bool,
    pub progress: Option<Progress>,
    pub outcome: Option<Result<PublishReport, String>>,
    /// The addon folder sent, linked to the item once it is published.
    pub folder: Option<PathBuf>,
}

impl AppState {
    /// Validates and uploads (or only validates, for `dry_run`), with
    /// Steam's progress shown as it comes.
    pub fn publish(&mut self, form: PublishForm, dry_run: bool, cx: &mut Context<Self>) {
        let app = self.tools.steam.game.ok_or_else(|| NO_GAME.to_owned());
        let spec = match app.and_then(|a| form.spec(a)) {
            Ok(spec) => spec,
            Err(why) => return self.published(Err(why), cx),
        };
        let folder = spec.content.clone().filter(|p| p.is_dir());
        self.tools.publish = PublishRun {
            running: true,
            dry_run,
            folder,
            ..PublishRun::default()
        };
        let (tx, rx) = async_channel::unbounded::<Progress>();
        let publish = self.tools.backend.publish;
        let work = cx
            .background_executor()
            .spawn(async move { publish(&spec, dry_run, &mut |p| drop(tx.try_send(p.clone()))) });
        cx.spawn(async move |this, cx| {
            while let Ok(p) = rx.recv().await {
                if this.update(cx, |s, cx| s.publish_progress(p, cx)).is_err() {
                    return;
                }
            }
            let result = work.await;
            let _ = this.update(cx, |s, cx| s.published(result, cx));
        })
        .detach();
        cx.notify();
    }
}

impl AppState {
    fn publish_progress(&mut self, p: Progress, cx: &mut Context<Self>) {
        self.tools.publish.progress = Some(p);
        cx.notify();
    }

    fn published(&mut self, result: Result<PublishReport, String>, cx: &mut Context<Self>) {
        let run = &mut self.tools.publish;
        run.running = false;
        let toast = match &result {
            Ok(_) if run.dry_run => (ToastKind::Info, "Dry run passed".to_owned()),
            Ok(_) => (ToastKind::Info, "Published to the Workshop".to_owned()),
            Err(why) => (ToastKind::Error, why.clone()),
        };
        let sent = match (&result, run.dry_run) {
            (Ok(r), false) => r.item_id.map(|id| (id, run.folder.take())),
            _ => None,
        };
        run.outcome = Some(result);
        self.toast(toast.0, toast.1, cx);
        if let Some((id, folder)) = sent {
            if let Some(dir) = folder {
                self.link_folder(id, dir, cx);
            }
            // The list and its numbers changed on Steam.
            self.refresh_steam(cx);
        }
    }
}
