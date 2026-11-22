use std::sync::mpsc::channel;

use steamworks::{AppId, Client, FileType, PublishedFileId, UpdateHandle};

use super::apply::apply;
use super::wait::{init, wait};
use crate::spec::{stage_content, ItemSpec};
use crate::{Progress, PublishReport, WorkshopError};

fn steam_err(e: steamworks::SteamError) -> WorkshopError {
    WorkshopError::Steam(e.to_string())
}

/// The id plus whether Steam asks for the legal agreement.
fn create(client: &Client, app: AppId) -> Result<(u64, bool), WorkshopError> {
    let (tx, rx) = channel();
    client
        .ugc()
        .create_item(app, FileType::Community, move |r| {
            let _ = tx.send(r);
        });
    let (id, legal) = wait(client, &rx, || {})?.map_err(steam_err)?;
    Ok((id.0, legal))
}

/// Creates the item first when the spec has no id, then one update
/// carries every field; the staged GMA folder lives until it returns.
pub fn publish(
    spec: &ItemSpec,
    progress: &mut dyn FnMut(&Progress),
) -> Result<PublishReport, WorkshopError> {
    let client = init(spec.app_id)?;
    let app = AppId(spec.app_id);
    let staged = match &spec.content {
        Some(c) => stage_content(spec.app_id, c)?,
        None => None,
    };
    let content = staged.as_ref().map(|d| d.path());
    let content = content.or(spec.content.as_deref());
    let ((id, legal_create), created) = match spec.item_id {
        Some(id) => ((id, false), false),
        None => (create(&client, app)?, true),
    };
    let handle = apply(
        client.ugc().start_item_update(app, PublishedFileId(id)),
        spec,
        content,
    );
    let legal = submit(&client, handle, spec.change_note.as_deref(), progress)?;
    Ok(PublishReport {
        dry_run: false,
        item_id: Some(id),
        created,
        needs_legal_agreement: legal || legal_create,
    })
}

/// Returns whether the owner still has to accept the legal agreement.
fn submit(
    client: &Client,
    handle: UpdateHandle,
    note: Option<&str>,
    progress: &mut dyn FnMut(&Progress),
) -> Result<bool, WorkshopError> {
    let (tx, rx) = channel();
    let watch = handle.submit(note, move |r| {
        let _ = tx.send(r);
    });
    let result = wait(client, &rx, || {
        let (stage, done, total) = watch.progress();
        let stage = format!("{stage:?}");
        progress(&Progress { stage, done, total });
    })?;
    Ok(result.map_err(steam_err)?.1)
}
