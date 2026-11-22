use gpui::{canvas, div, prelude::*, Context, Corners, Rgba, Window};

use super::frame::native;
use super::{Colors, Status, Viewport};
use crate::theme::prybar::{tokens, Light, Tokens};

impl Render for Viewport {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = tokens(cx.try_global::<Light>().is_some_and(|l| l.0));
        let colors = colors(&t);
        let surface = surface(self, cx.entity(), colors);
        div()
            .debug_selector(|| "pb-viewport".into())
            .relative()
            .flex_1()
            .min_h_48()
            .rounded_lg()
            .border_1()
            .border_color(t.border)
            .bg(t.surface)
            .overflow_hidden()
            .child(surface.absolute().size_full())
            .child(label(self, &t))
    }
}

fn colors(t: &Tokens) -> Colors {
    let c = |r: Rgba| [r.r, r.g, r.b, r.a];
    let mut grid = c(t.border);
    grid[3] = 0.9;
    let mut wire = c(t.text);
    wire[3] = 0.28;
    Colors {
        clear: c(t.surface),
        grid,
        wire,
    }
}

/// Requests a frame at the device size of the region at prepaint,
/// paints the newest one, and frees replaced ones from the atlas.
fn surface(v: &mut Viewport, this: gpui::Entity<Viewport>, colors: Colors) -> gpui::Canvas<()> {
    let image = v.image.clone();
    let drops = std::mem::take(&mut v.drops);
    canvas(
        move |b, window, cx| {
            let s = window.scale_factor();
            let w = (f32::from(b.size.width) * s).round().max(1.0) as u32;
            let h = (f32::from(b.size.height) * s).round().max(1.0) as u32;
            this.update(cx, |v, _| v.request(w, h, colors));
        },
        move |b, _, window, _| {
            for d in drops {
                let _ = window.drop_image(d);
            }
            if let Some(img) = image {
                let at = native(b, &img, window.scale_factor());
                let _ = window.paint_image(b, at, Corners::default(), img, 0, false);
            }
        },
    )
}

/// Status while there is nothing to see, numbers once there is.
fn label(v: &Viewport, t: &Tokens) -> impl IntoElement {
    let (text, color) = match &v.status {
        Status::Idle => ("Open a .mdl to preview it".to_string(), t.muted),
        Status::Loading => ("Loading model".to_string(), t.muted),
        Status::Failed(e) => (e.clone(), t.danger),
        Status::Ready => (
            format!("{} triangles · {:.1} ms", v.triangles, v.ms),
            t.muted,
        ),
    };
    let ready = v.status == Status::Ready;
    let place = div().absolute().text_xs().text_color(color);
    let place = if ready {
        place.right_3().bottom_2()
    } else {
        place.size_full().flex().items_center().justify_center()
    };
    place.child(text)
}
