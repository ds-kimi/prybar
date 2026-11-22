use std::path::PathBuf;
use std::sync::Arc;

use gpui::{AppContext as _, Context, Task};
use prybar_mdl::ir::ModelIr;

use super::{Status, Viewport};
use crate::viewport::camera::Orbit;
use crate::viewport::mesh::{self, Pick, Scene};
use crate::viewport::render::Worker;

type Built = Result<(Arc<ModelIr>, Scene), String>;

/// Parses and builds the mesh in the background: big models take a
/// moment and a broken one must only produce a message.
pub fn start(path: PathBuf, cx: &mut Context<Viewport>) -> Task<()> {
    let job = cx.background_spawn(async move {
        let ir = prybar_service::model::open(&path.to_string_lossy()).map_err(|e| e.to_string())?;
        let scene = mesh::build(&ir, &Pick::default())?;
        Ok((Arc::new(ir), scene))
    });
    cx.spawn(async move |this, cx| {
        let built = job.await;
        let _ = this.update(cx, |v, cx| loaded(v, built, true, cx));
    })
}

/// Builds another body group, skin or LOD of the open model; the camera
/// stays where the user put it.
pub fn rebuild(ir: Arc<ModelIr>, pick: Pick, cx: &mut Context<Viewport>) -> Task<()> {
    let job = cx.background_spawn(async move {
        let scene = mesh::build(&ir, &pick)?;
        Ok((ir, scene))
    });
    cx.spawn(async move |this, cx| {
        let built = job.await;
        let _ = this.update(cx, |v, cx| loaded(v, built, false, cx));
    })
}

fn loaded(v: &mut Viewport, built: Built, refit: bool, cx: &mut Context<Viewport>) {
    let (ir, scene) = match built {
        Ok(b) => b,
        Err(e) => {
            v.status = Status::Failed(e);
            return cx.notify();
        }
    };
    if refit {
        v.fit = Some(Orbit::fit(&scene.bounds));
        v.camera = v.fit;
    }
    v.triangles = scene.triangles();
    if v.worker.is_none() {
        let worker = Worker::spawn();
        // The pump lives as long as the worker: first in `tasks`.
        v.tasks.insert(0, pump(worker.rx.clone(), cx));
        v.worker = Some(worker);
    }
    (v.ir, v.bind) = (Some(ir), Some(Arc::new(scene)));
    v.send();
    cx.notify();
}

/// Moves render thread answers onto the UI thread as they arrive.
fn pump(
    rx: async_channel::Receiver<crate::viewport::render::Output>,
    cx: &mut Context<Viewport>,
) -> Task<()> {
    cx.spawn(async move |this, cx| {
        while let Ok(msg) = rx.recv().await {
            if this.update(cx, |v, cx| v.accept(msg, cx)).is_err() {
                break;
            }
        }
    })
}
