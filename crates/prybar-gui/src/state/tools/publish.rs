//! A Workshop upload: the form as typed, its progress, its result.

use gpui::Context;
use prybar_workshop::spec::{ItemSpec, Visibility, GMOD_APP_ID};
use prybar_workshop::{Progress, PublishReport};

use crate::state::{AppState, ToastKind};

/// The publish fields as text, straight from the inputs.
#[derive(Debug, Clone, Default)]
pub struct PublishForm {
    pub item_id: String,
    pub title: String,
    pub description: String,
    pub tags: String,
    pub visibility: String,
    pub preview: String,
    pub content: String,
    pub change_note: String,
}

#[derive(Default)]
pub struct PublishRun {
    pub running: bool,
    pub dry_run: bool,
    pub progress: Option<Progress>,
    pub outcome: Option<Result<PublishReport, String>>,
}

impl PublishForm {
    /// Empty fields stay unset, so an update only changes what was typed.
    pub fn spec(&self) -> Result<ItemSpec, String> {
        let some = |s: &str| Some(s.trim().to_owned()).filter(|s| !s.is_empty());
        let item_id = match some(&self.item_id) {
            None => None,
            Some(id) => Some(
                id.parse()
                    .map_err(|_| "The item id is the number in the item's Workshop link.")?,
            ),
        };
        let visibility = match self.visibility.trim().to_ascii_lowercase().as_str() {
            "public" => Visibility::Public,
            "friends" => Visibility::FriendsOnly,
            "unlisted" => Visibility::Unlisted,
            _ => Visibility::Private,
        };
        let tags = self.tags.split(',').filter_map(some).collect();
        Ok(ItemSpec {
            app_id: GMOD_APP_ID,
            item_id,
            title: some(&self.title),
            description: some(&self.description),
            tags,
            visibility: Some(visibility),
            preview: some(&self.preview).map(Into::into),
            content: some(&self.content).map(Into::into),
            change_note: some(&self.change_note),
        })
    }
}

impl AppState {
    /// Validates and uploads (or only validates, for `dry_run`), with
    /// Steam's progress shown as it comes.
    pub fn publish(&mut self, form: PublishForm, dry_run: bool, cx: &mut Context<Self>) {
        let spec = match form.spec() {
            Ok(spec) => spec,
            Err(why) => return self.published(Err(why), cx),
        };
        self.tools.publish = PublishRun {
            running: true,
            dry_run,
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
        run.outcome = Some(result);
        self.toast(toast.0, toast.1, cx);
    }
}
