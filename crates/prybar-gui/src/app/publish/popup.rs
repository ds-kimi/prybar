//! The create and edit popups, hosted by the shared modal.

use gpui::{div, prelude::*, Context, Entity, Subscription, WeakEntity, Window};
use prybar_workshop::ListedItem;

use super::super::Root;
use crate::designs::modal::Modal;
use crate::theme::prybar::tokens;

/// The popup's body or its pinned footer. It draws through the root
/// because the form, its inputs and its handlers all live there.
pub struct PublishPopup {
    root: WeakEntity<Root>,
    footer: bool,
    _subs: Vec<Subscription>,
}

impl Render for PublishPopup {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (footer, wide) = (
            self.footer,
            crate::designs::modal::wide(window.viewport_size()),
        );
        let Some(root) = self.root.upgrade() else {
            return div().into_any_element();
        };
        root.update(cx, |r, cx| {
            let state = r.state.clone();
            let s = state.read(cx);
            let t = tokens(s.light);
            match footer {
                true => crate::designs::publish_footer(s, &r.forms, r.editing.is_some(), t, cx),
                false => {
                    crate::designs::publish_popup(s, &r.forms, r.editing.as_ref(), wide, t, cx)
                }
            }
        })
    }
}

impl Root {
    /// Fills the form from `item` (or empties it for a new one) and opens
    /// the popup; closing with edits asks first.
    pub fn open_publish(
        &mut self,
        item: Option<ListedItem>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match &item {
            Some(it) => self.edit_item(it, cx),
            None => self.new_item(cx),
        }
        let title = item.as_ref().map_or("New Workshop item".to_owned(), |i| {
            format!("Edit {}", i.title)
        });
        self.editing = item;
        self.state
            .update(cx, |s, _| s.tools.publish = Default::default());
        let start = self.forms.publish.form(cx);
        let inputs = self.forms.publish.clone();
        let (body, footer) = (popup(self, false, cx), popup(self, true, cx));
        let dirty = move |cx: &gpui::App| inputs.form(cx) != start;
        let modal = Modal::new(title, body.into(), Some(footer.into()), dirty, cx);
        self.show_modal(modal, window, cx);
    }
}

/// Redraws on any state or form change, since the body reads both.
fn popup(root: &Root, footer: bool, cx: &mut Context<Root>) -> Entity<PublishPopup> {
    let (state, weak) = (root.state.clone(), cx.entity().downgrade());
    let inputs: Vec<_> = root.forms.publish.all().into_iter().cloned().collect();
    cx.new(|cx| {
        let mut subs = vec![cx.observe(&state, |_, _, cx| cx.notify())];
        subs.extend(inputs.iter().map(|i| cx.observe(i, |_, _, cx| cx.notify())));
        PublishPopup {
            root: weak,
            footer,
            _subs: subs,
        }
    })
}
