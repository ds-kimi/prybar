use std::path::PathBuf;

use gpui::{AppContext as _, Context, Task};

use super::{Status, Viewport};
use crate::viewport::camera::Orbit;
use crate::viewport::mesh::{self, Scene};
use crate::viewport::render::Worker;

/// Parses and builds the mesh in the background: big models take a
/// moment and a broken one must only produce a message.
pub fn start(path: PathBuf, cx: &mut Context<Viewport>) -> Task<()> {
    let job = cx.background_spawn(async move { build(&path) });
    cx.spawn(async move |this, cx| {
        let scene = job.await;
        let _ = this.update(cx, |v, cx| loaded(v, scene, cx));
    })
}

fn build(path: &std::path::Path) -> Result<Scene, String> {
    let ir = prybar_service::model::open(&path.to_string_lossy()).map_err(|e| e.to_string())?;
    mesh::build(&ir, &[], 0)
}

fn loaded(v: &mut Viewport, scene: Result<Scene, String>, cx: &mut Context<Viewport>) {
    let scene = match scene {
        Ok(s) => s,
        Err(e) => {
            v.status = Status::Failed(e);
            return cx.notify();
        }
    };
    v.camera = Some(Orbit::fit(&scene.bounds));
    v.triangles = scene.triangles();
    if v.worker.is_none() {
        let worker = Worker::spawn();
        // The pump lives as long as the worker: first in `tasks`.
        v.tasks.insert(0, pump(worker.rx.clone(), cx));
        v.worker = Some(worker);
    }
    if let Some(w) = &v.worker {
        w.scene(scene);
    }
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
