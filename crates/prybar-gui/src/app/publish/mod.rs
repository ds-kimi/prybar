//! The Workshop form's fields, kept together so the form reads them
//! in one place.

use gpui::{App, Context, Entity, Subscription};

mod desc;
mod fill;
mod items;
mod popup;
mod visibility;

pub use items::ItemsInputs;

use super::forms::field;
use super::Root;
use crate::designs::bbcode::BbEditor;
use crate::designs::input::TextInput;
use crate::designs::select::Select;
use crate::state::tools::PublishForm;
use crate::state::{checks, AppState};
use visibility::{visibility, VALUES};

#[derive(Clone)]
pub struct PublishInputs {
    pub item_id: Entity<TextInput>,
    pub title: Entity<TextInput>,
    pub description: Entity<TextInput>,
    /// The BBCode toolbar and preview around the description.
    pub editor: Entity<BbEditor>,
    pub tags: Entity<TextInput>,
    pub visibility: Entity<Select>,
    pub content: Entity<TextInput>,
    pub preview: Entity<TextInput>,
    pub change_note: Entity<TextInput>,
    /// The toolbar above the user's items.
    pub items: ItemsInputs,
}

impl PublishInputs {
    pub fn new(cx: &mut App) -> Self {
        let description = desc::field(cx);
        let editor = desc::editor(&description, cx);
        PublishInputs {
            item_id: field(cx, "Empty for a new item", "", false, checks::item_id),
            title: field(cx, "Addon name", "", false, checks::title),
            description,
            editor,
            tags: field(cx, "Empty: from addon.json", "", false, checks::tags),
            visibility: visibility(cx),
            content: field(cx, "Addon folder or .gma", "", false, checks::existing),
            preview: field(cx, "/path/to/preview.jpg", "", false, checks::preview),
            change_note: field(cx, "What changed", "", true, checks::description),
            items: ItemsInputs::new(cx),
        }
    }

    pub fn all(&self) -> [&Entity<TextInput>; 7] {
        [
            &self.item_id,
            &self.title,
            &self.description,
            &self.tags,
            &self.content,
            &self.preview,
            &self.change_note,
        ]
    }

    /// The inputs as plain text for the state to turn into a spec.
    pub fn form(&self, cx: &App) -> PublishForm {
        let text = |e: &Entity<TextInput>| e.read(cx).text().to_owned();
        PublishForm {
            item_id: text(&self.item_id),
            title: text(&self.title),
            description: text(&self.description),
            tags: text(&self.tags),
            visibility: VALUES
                .get(self.visibility.read(cx).current())
                .map_or("private", |v| v)
                .into(),
            preview: text(&self.preview),
            content: text(&self.content),
            change_note: text(&self.change_note),
            editing: None,
        }
    }

    /// Typing or picking a content path re-checks it when it is an
    /// addon folder, so rejected files show before any upload.
    pub fn watch(&self, state: &Entity<AppState>, cx: &mut Context<Root>) -> Vec<Subscription> {
        let mut subs = self.items.watch(state, cx);
        let state = state.clone();
        subs.push(cx.observe(&self.content, move |_, input, cx| {
            let path = input.read(cx).text().to_owned();
            state.update(cx, |s, cx| s.check_content(&path, cx));
        }));
        subs
    }
}
