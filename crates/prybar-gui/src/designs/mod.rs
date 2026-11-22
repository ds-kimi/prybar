//! Design views: each design renders the same state its own way.

mod canvas;
pub mod console;
mod placeholder;
mod prybar;
mod workbench;

use gpui::{AnyElement, Context, Entity};

use crate::app::Root;
use crate::state::{AppState, DesignKind};

pub fn render(kind: DesignKind, state: Entity<AppState>, cx: &mut Context<Root>) -> AnyElement {
    match kind {
        DesignKind::Workbench => workbench::render(state, cx),
        DesignKind::Canvas => canvas::render(state, cx),
        DesignKind::Console => console::render(state, cx),
        DesignKind::Prybar => prybar::render(state, cx),
    }
}
