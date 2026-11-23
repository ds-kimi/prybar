//! Images in the preview, fetched once each through the same polite,
//! cached downloader as item previews, never on the drawing thread.

use gpui::{AppContext as _, Context};
use prybar_workshop::bbcode::{Node, Tag};

use super::render::text_of;
use super::BbEditor;

/// Where one image link stands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Shot {
    Loading,
    Ready(std::path::PathBuf),
    Failed,
}

/// Every image link in the tree, in order.
fn links(nodes: &[Node], out: &mut Vec<String>) {
    for n in nodes {
        if let Node::Tag(tag, _, kids) = n {
            if *tag == Tag::Img {
                out.push(text_of(kids).trim().to_owned());
            } else {
                links(kids, out);
            }
        }
    }
}

impl BbEditor {
    /// Starts a download for each link not seen yet; the preview draws
    /// again when one lands.
    pub(super) fn want(&mut self, nodes: &[Node], cx: &mut Context<Self>) {
        let mut urls = Vec::new();
        links(nodes, &mut urls);
        for url in urls {
            if self.shots.contains_key(&url) {
                continue;
            }
            self.shots.insert(url.clone(), Shot::Loading);
            let fetch = self.fetch.clone();
            let key = url.clone();
            let job = cx.background_spawn(async move { fetch(&url) });
            cx.spawn(async move |this, cx| {
                let shot = job.await.map_or(Shot::Failed, Shot::Ready);
                let _ = this.update(cx, |e, cx| {
                    e.shots.insert(key, shot);
                    cx.notify();
                });
            })
            .detach();
        }
    }

    pub fn shots(&self) -> &std::collections::HashMap<String, Shot> {
        &self.shots
    }
}
