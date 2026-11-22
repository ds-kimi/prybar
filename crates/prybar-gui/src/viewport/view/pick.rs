use gpui::Context;

use super::{load, Viewport};
use crate::viewport::mesh::Pick;

impl Viewport {
    /// Shows another body group, skin or LOD of the open model.
    pub fn repick(&mut self, pick: Pick, cx: &mut Context<Self>) {
        if pick == self.pick {
            return;
        }
        self.pick = pick.clone();
        if let Some(ir) = self.ir.clone() {
            self.rebuild = Some(load::rebuild(ir, pick, cx));
        }
        cx.notify();
    }
}
