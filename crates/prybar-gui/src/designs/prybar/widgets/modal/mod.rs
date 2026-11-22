//! A centered popup over a dimmed window. Escape, a click outside or the
//! close button ask to close; with unsaved changes the popup asks first.
//! Tab stays inside while it is open.

// The publish popups open it; until they land only tests do.
#![cfg_attr(not(test), allow(dead_code))]

mod keys;
mod view;

use std::rc::Rc;
use std::time::Duration;

use gpui::{
    AnyView, App, AppContext as _, Context, Entity, EventEmitter, FocusHandle, SharedString,
};

/// Long enough to read as motion, short enough to never wait on.
pub const EXIT: Duration = Duration::from_millis(120);

/// Sent once the exit animation is done and the host should drop it.
pub struct Closed;

pub struct Modal {
    title: SharedString,
    body: AnyView,
    focus: FocusHandle,
    /// Asked on every close request; `true` shows the discard prompt.
    dirty: Rc<dyn Fn(&App) -> bool>,
    confirming: bool,
    closing: bool,
}

impl EventEmitter<Closed> for Modal {}

impl Modal {
    pub fn new(
        title: impl Into<SharedString>,
        body: AnyView,
        dirty: impl Fn(&App) -> bool + 'static,
        cx: &mut App,
    ) -> Entity<Modal> {
        let title = title.into();
        cx.new(|cx| Modal {
            title,
            body,
            focus: cx.focus_handle(),
            dirty: Rc::new(dirty),
            confirming: false,
            closing: false,
        })
    }

    pub fn focus_handle(&self) -> &FocusHandle {
        &self.focus
    }

    /// Closes at once when nothing would be lost, else asks once; a second
    /// request while asking is the user insisting, so it closes.
    pub fn request_close(&mut self, cx: &mut Context<Self>) {
        if !self.confirming && (self.dirty)(cx) {
            self.confirming = true;
            return cx.notify();
        }
        self.close(cx)
    }

    /// Plays the exit, then tells the host.
    pub fn close(&mut self, cx: &mut Context<Self>) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        cx.notify();
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(EXIT).await;
            let _ = this.update(cx, |_, cx| cx.emit(Closed));
        })
        .detach();
    }
}
