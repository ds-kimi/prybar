//! What a click on a listed item or on New item does to the form.

use gpui::{Context, Entity};
use prybar_workshop::ListedItem;

use super::super::Root;
use super::items::SHOWN;
use crate::designs::input::TextInput;

impl Root {
    /// Fills the form from a listed item, so Publish updates it.
    pub fn edit_item(&mut self, item: &ListedItem, cx: &mut Context<Self>) {
        let f = self.forms.publish.clone();
        let text = |e: &Entity<TextInput>, v: &str, cx: &mut Context<Self>| {
            e.update(cx, |i, cx| i.set_text(v, cx))
        };
        text(&f.item_id, &item.item_id.to_string(), cx);
        text(&f.title, &item.title, cx);
        let ix = SHOWN
            .iter()
            .position(|v| *v == item.visibility)
            .unwrap_or(2);
        f.visibility.update(cx, |s, cx| s.pick(ix, cx));
    }

    /// An empty form for a new item, hidden until the user says otherwise.
    pub fn new_item(&mut self, cx: &mut Context<Self>) {
        let f = self.forms.publish.clone();
        for e in [
            &f.item_id,
            &f.title,
            &f.description,
            &f.tags,
            &f.change_note,
        ] {
            e.update(cx, |i, cx| i.set_text("", cx));
        }
        f.visibility.update(cx, |s, cx| s.pick(2, cx));
    }
}
