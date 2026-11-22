use std::sync::mpsc;

use super::{Frame, Renderer, Request};
use crate::viewport::mesh::Scene;

/// What the UI asks of the render thread.
pub enum Cmd {
    Scene(Box<Scene>),
    Frame(Request),
    Materials(std::sync::Arc<Vec<Vec<prybar_vtf::Image>>>),
}

pub enum Output {
    /// The GPU opened; `wire` says if the wireframe toggle can work.
    Ready {
        wire: bool,
    },
    Frame(Frame),
    Error(String),
}

/// The renderer on its own thread: the UI sends requests and never
/// waits; when it falls behind only the newest request is drawn.
pub struct Worker {
    tx: mpsc::Sender<Cmd>,
    pub rx: async_channel::Receiver<Output>,
}

impl Worker {
    pub fn spawn() -> Self {
        let (tx, cmds) = mpsc::channel();
        let (out, rx) = async_channel::unbounded();
        let spawned = std::thread::Builder::new()
            .name("viewport".into())
            .spawn(move || run(&cmds, &out));
        if let Err(e) = spawned {
            let (out, rx) = async_channel::bounded(1);
            let _ = out.try_send(Output::Error(format!("No render thread: {e}")));
            return Self { tx, rx };
        }
        Self { tx, rx }
    }

    /// Never blocks; the render thread drains everything queued and
    /// draws only the newest frame request.
    pub fn send(&self, cmd: Cmd) {
        let _ = self.tx.send(cmd);
    }
}

/// Ends when the `Worker` is dropped and its sender with it.
fn run(cmds: &mpsc::Receiver<Cmd>, out: &async_channel::Sender<Output>) {
    let mut r = match Renderer::new() {
        Ok(r) => r,
        Err(e) => return drop(out.try_send(Output::Error(e))),
    };
    let _ = out.try_send(Output::Ready { wire: r.gpu.wire });
    while let Ok(first) = cmds.recv() {
        let mut req = None;
        for cmd in std::iter::once(first).chain(cmds.try_iter()) {
            match cmd {
                Cmd::Scene(s) => r.set_scene(&s),
                Cmd::Frame(q) => req = Some(q),
                Cmd::Materials(m) => r.materials.set(&r.gpu, &m),
            }
        }
        if let Some(q) = req {
            let msg = r.render(&q).map_or_else(Output::Error, Output::Frame);
            if out.try_send(msg).is_err() {
                return;
            }
        }
    }
}
