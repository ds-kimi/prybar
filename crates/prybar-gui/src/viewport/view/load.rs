use std::path::PathBuf;
use std::sync::Arc;

use gpui::{AppContext as _, Context, Task};
use prybar_mdl::ir::ModelIr;

use super::{Status, Viewport};
use crate::viewport::camera::Orbit;
use crate::viewport::mesh::{self, Pick, Scene};
use crate::viewport::render::{Cmd, Worker};

type Built = Result<(Arc<ModelIr>, Scene), String>;

/// Parses and builds the mesh in the background: big models take a
/// moment and a broken one must only produce a message.
/// Textures come second: opening every game VPK can take a moment and
/// the clay model is already worth looking at.
pub fn start(path: PathBuf, cx: &mut Context<Viewport>) -> Task<()> {
    let model = path.clone();
    let job = cx.background_spawn(async move {
        let ir =
            prybar_service::model::open(&model.to_string_lossy()).map_err(|e| e.to_string())?;
        let scene = mesh::build(&ir, &Pick::default())?;
        Ok((Arc::new(ir), scene))
    });
    cx.spawn(async move |this, cx| {
        let built = job.await;
        let ir = built.as_ref().ok().map(|(ir, _)| ir.clone());
        let _ = this.update(cx, |v, cx| loaded(v, built, true, cx));
        let Some(ir) = ir else { return };
        let set = cx
            .background_spawn(async move { crate::textures::load(&path, &ir) })
            .await;
        let _ = this.update(cx, |v, cx| v.textured(set, cx));
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
        v.tasks.insert(0, super::frame::pump(worker.rx.clone(), cx));
        v.worker = Some(worker);
    }
    (v.ir, v.bind) = (Some(ir), Some(Arc::new(scene)));
    v.send();
    cx.notify();
}

impl Viewport {
    /// Hands the base textures to the renderer; a model with none of them
    /// found still shows its checkers, which says what is missing.
    fn textured(&mut self, set: Arc<crate::textures::TextureSet>, cx: &mut Context<Self>) {
        if let Some(w) = &self.worker {
            w.send(Cmd::Materials(set.gpu.clone()));
        }
        self.textures = Some(set);
        self.last = None;
        cx.notify();
    }
}
