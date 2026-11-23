//! Toast lifetimes: confirmations leave on their own unless hovered,
//! errors stay until dismissed, and every exit is played before removal.

use std::time::Duration;

use gpui::Context;

use super::{AppState, Toast, ToastKind};

/// Past this many on screen, the oldest slides away.
pub const MAX_VISIBLE: usize = 4;
/// How long a confirmation stays when nobody points at it.
pub const LIFE: Duration = Duration::from_secs(4);
/// Room opens for it, then it slides in from the right.
pub const ENTER: Duration = Duration::from_millis(280);
/// Slide out, then the gap closes so the others glide into place.
pub const LEAVE: Duration = Duration::from_millis(280);

impl AppState {
    pub fn toast(&mut self, kind: ToastKind, text: String, cx: &mut Context<Self>) {
        self.next_id += 1;
        let id = self.next_id;
        let (leaving, hovered, epoch) = (false, false, 0);
        self.toasts.push(Toast {
            id,
            kind,
            text,
            leaving,
            hovered,
            epoch,
        });
        let shown: Vec<u64> = self
            .toasts
            .iter()
            .filter(|t| !t.leaving)
            .map(|t| t.id)
            .collect();
        for old in &shown[..shown.len().saturating_sub(MAX_VISIBLE)] {
            self.dismiss_toast(*old, cx);
        }
        if kind == ToastKind::Info {
            self.arm_toast(id, cx);
        }
        cx.notify();
    }

    /// A fresh full life, so a toast read under the pointer is not gone
    /// the instant the pointer leaves.
    fn arm_toast(&mut self, id: u64, cx: &mut Context<Self>) {
        let Some(epoch) = self.toasts.iter().find(|t| t.id == id).map(|t| t.epoch) else {
            return;
        };
        let timer = cx.background_executor().timer(LIFE);
        cx.spawn(async move |this, cx| {
            timer.await;
            let _ = this.update(cx, |s, cx| {
                let due = s
                    .toasts
                    .iter()
                    .any(|t| t.id == id && t.epoch == epoch && !t.hovered);
                if due {
                    s.dismiss_toast(id, cx);
                }
            });
        })
        .detach();
    }

    pub fn hover_toast(&mut self, id: u64, hovered: bool, cx: &mut Context<Self>) {
        let Some(t) = self.toasts.iter_mut().find(|t| t.id == id) else {
            return;
        };
        t.hovered = hovered;
        t.epoch = t.epoch.wrapping_add(1);
        if !hovered && t.kind == ToastKind::Info && !t.leaving {
            self.arm_toast(id, cx);
        }
        cx.notify();
    }

    /// Starts the exit; a second call while it plays does nothing.
    pub fn dismiss_toast(&mut self, id: u64, cx: &mut Context<Self>) {
        match self.toasts.iter_mut().find(|t| t.id == id) {
            Some(t) if !t.leaving => t.leaving = true,
            _ => return,
        }
        cx.notify();
        let timer = cx.background_executor().timer(LEAVE);
        cx.spawn(async move |this, cx| {
            timer.await;
            let _ = this.update(cx, |s, cx| {
                s.toasts.retain(|t| t.id != id);
                cx.notify();
            });
        })
        .detach();
    }
}
