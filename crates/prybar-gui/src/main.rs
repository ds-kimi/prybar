//! Desktop interface for prybar.

mod hello;

use gpui::{px, size, App, AppContext, Bounds, WindowBounds, WindowOptions};

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(640.), px(420.)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };
        // A window that fails to open leaves nothing to show, so quit cleanly.
        if cx
            .open_window(options, |_, cx| cx.new(|_| hello::Hello::default()))
            .is_err()
        {
            cx.quit();
        }
    });
}
