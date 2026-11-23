//! The actions bar pinned under the scrolling body.

use gpui::{div, prelude::*, Div};

use super::Modal;
use crate::theme::prybar::Tokens;

impl Modal {
    /// Never shrinks, so the actions stay in reach however long the form.
    pub(super) fn footer_bar(&self, t: Tokens) -> Option<Div> {
        let f = self.footer.clone()?;
        let bar = div().flex_none().p_4().border_t_1().border_color(t.border);
        Some(bar.debug_selector(|| "pb-modal-footer".into()).child(f))
    }
}
