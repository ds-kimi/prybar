//! The toolbar above "my items": a search field and two selects whose
//! choices flow into the state's query as they change.

use gpui::{App, Context, Entity, Subscription};
use prybar_workshop::catalog::Sort;
use prybar_workshop::spec::Visibility;

use super::super::forms::field;
use super::super::Root;
use crate::designs::input::TextInput;
use crate::designs::select::{Choice, Select};
use crate::state::tools::Catalog;
use crate::state::AppState;

/// The visibility filter's choices after "Any", in [`super::visibility`] order.
pub(super) const SHOWN: [Visibility; 4] = [
    Visibility::Public,
    Visibility::FriendsOnly,
    Visibility::Private,
    Visibility::Unlisted,
];

#[derive(Clone)]
pub struct ItemsInputs {
    pub search: Entity<TextInput>,
    pub sort: Entity<Select>,
    pub filter: Entity<Select>,
}

impl ItemsInputs {
    pub fn new(cx: &mut App) -> Self {
        let c = |label: &'static str, tip: &'static str| Choice {
            label: label.into(),
            tip: tip.into(),
        };
        let sorts = vec![
            c("Most subscribed", "Items with the most subscribers"),
            c("Best rated", "Highest share of up votes"),
            c("Most favorited", "Items most often added to favorites"),
            c("Recently updated", "Last changed on Steam"),
            c("Newest", "Most recently created"),
            c("Title", "By name"),
        ];
        let filters = vec![
            c("Any visibility", "Show every item"),
            c("Public", "Only items everyone can see"),
            c("Friends only", "Only items your friends can see"),
            c("Hidden", "Only items only you can see"),
            c("Unlisted", "Only items reachable by link"),
        ];
        ItemsInputs {
            search: field(cx, "Search title, tag or id", "", false, |_| None),
            sort: Select::new("pb-items-sort", sorts, 3, cx),
            filter: Select::new("pb-items-filter", filters, 0, cx),
        }
    }

    /// Starts the sort select on the saved choice, then keeps the query
    /// in step with every keystroke and pick.
    pub fn watch(&self, state: &Entity<AppState>, cx: &mut Context<Root>) -> Vec<Subscription> {
        let saved = state.read(cx).tools.steam.catalog.query.sort;
        let ix = Sort::ALL.iter().position(|s| *s == saved).unwrap_or(3);
        self.sort.update(cx, |s, cx| s.pick(ix, cx));
        let text = |i: &TextInput| i.text().to_owned();
        let sort = |s: &Select| Sort::ALL.get(s.current()).copied().unwrap_or_default();
        let vis = |s: &Select| {
            s.current()
                .checked_sub(1)
                .and_then(|i| SHOWN.get(i))
                .copied()
        };
        vec![
            sync(cx, &self.search, state, text, |c| &mut c.query.text),
            sync(cx, &self.sort, state, sort, |c| &mut c.query.sort),
            sync(cx, &self.filter, state, vis, |c| &mut c.query.visibility),
        ]
    }
}

/// Copies what `read` finds in the input into the catalog field `slot`
/// points at, only when it changed: observers also fire on focus and
/// hover redraws.
fn sync<E: 'static, V: PartialEq + 'static>(
    cx: &mut Context<Root>,
    input: &Entity<E>,
    state: &Entity<AppState>,
    read: fn(&E) -> V,
    slot: fn(&mut Catalog) -> &mut V,
) -> Subscription {
    let state = state.clone();
    cx.observe(input, move |_, input, cx| {
        let v = read(input.read(cx));
        state.update(cx, |s, cx| {
            if *slot(&mut s.tools.steam.catalog) != v {
                s.set_items(|c| *slot(c) = v, cx);
            }
        });
    })
}
