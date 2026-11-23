//! The publish popup's fields as typed, turned into what Steam is sent.

use prybar_workshop::item::{changes, Draft};
use prybar_workshop::spec::{ItemSpec, Visibility, GMOD_APP_ID};
use prybar_workshop::ListedItem;

/// The publish fields as text, straight from the inputs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PublishForm {
    pub item_id: String,
    pub title: String,
    pub description: String,
    pub tags: String,
    pub visibility: String,
    pub preview: String,
    pub content: String,
    pub change_note: String,
    /// The listed item the edit popup opened on; its id matching
    /// `item_id` means only the changed fields are sent.
    pub editing: Option<ListedItem>,
}

fn some(s: &str) -> Option<String> {
    Some(s.trim().to_owned()).filter(|s| !s.is_empty())
}

impl PublishForm {
    /// Empty fields stay unset, so an update only changes what was typed.
    pub fn spec(&self) -> Result<ItemSpec, String> {
        let item_id = match some(&self.item_id) {
            None => None,
            Some(id) => Some(
                id.parse()
                    .map_err(|_| "The item id is the number in the item's Workshop link.")?,
            ),
        };
        if let Some(item) = self.editing.as_ref().filter(|i| Some(i.item_id) == item_id) {
            return Ok(changes(GMOD_APP_ID, item, &self.draft(item)));
        }
        Ok(ItemSpec {
            app_id: GMOD_APP_ID,
            item_id,
            title: some(&self.title),
            description: some(&self.description),
            tags: self.tags.split(',').filter_map(some).collect(),
            visibility: Some(self.visibility()),
            preview: some(&self.preview).map(Into::into),
            content: some(&self.content).map(Into::into),
            change_note: some(&self.change_note),
        })
    }

    /// The fields over the item's current values; empty tags keep its tags.
    fn draft(&self, item: &ListedItem) -> Draft {
        let tags: Vec<String> = self.tags.split(',').filter_map(some).collect();
        Draft {
            title: self.title.clone(),
            description: self.description.clone(),
            tags: if tags.is_empty() {
                item.tags.clone()
            } else {
                tags
            },
            visibility: self.visibility(),
            preview: some(&self.preview).map(Into::into),
            content: some(&self.content).map(Into::into),
            change_note: self.change_note.clone(),
        }
    }
}

impl PublishForm {
    pub fn visibility(&self) -> Visibility {
        match self.visibility.trim().to_ascii_lowercase().as_str() {
            "public" => Visibility::Public,
            "friends" => Visibility::FriendsOnly,
            "unlisted" => Visibility::Unlisted,
            _ => Visibility::Private,
        }
    }
}
