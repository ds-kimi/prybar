use gpui::{canvas, div, prelude::*, Context, Corners, Rgba, Window};

use super::frame::native;
use super::{drag, input, overlay};
use super::{Colors, Status, Viewport};
use crate::theme::prybar::{tokens, Light, Tokens};

impl Render for Viewport {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = tokens(cx.try_global::<Light>().is_some_and(|l| l.0));
        let colors = colors(&t, self.backdrop);
        let surface = surface(self, cx.entity(), colors);
        let focus = self.focus.get_or_insert_with(|| cx.focus_handle()).clone();
        let el = input::keys(input::attach(div(), cx), cx);
        let ready = self.status == Status::Ready;
        el.track_focus(&focus)
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
            .children(ready.then(|| overlay::controls(self, &t, cx)))
            .children(ready.then(|| overlay::variants(self, &t, cx)))
            .children(ready.then(|| overlay::timeline(self, &t, cx)).flatten())
    }
}

fn colors(t: &Tokens, backdrop: bool) -> Colors {
    let c = |r: Rgba| [r.r, r.g, r.b, r.a];
    let mut grid = c(t.border);
    grid[3] = 0.9;
    let mut wire = c(t.text);
    wire[3] = 0.28;
    Colors {
        // Mid grey reads both dark and light silhouettes.
        clear: if backdrop {
            [0.36, 0.37, 0.39, 1.0]
        } else {
            c(t.surface)
        },
        grid,
        wire,
    }
}

/// Requests a frame at the device size of the region at prepaint,
/// paints the newest one, and frees replaced ones from the atlas.
type Surface = gpui::Canvas<gpui::Entity<Viewport>>;

fn surface(v: &mut Viewport, this: gpui::Entity<Viewport>, colors: Colors) -> Surface {
    let image = v.image.clone();
    let drops = std::mem::take(&mut v.drops);
    canvas(
        move |b, window, cx| {
            let px =
                |l: gpui::Pixels| (f32::from(l) * window.scale_factor()).round().max(1.0) as u32;
            let (w, h) = (px(b.size.width), px(b.size.height));
            this.update(cx, |v, _| {
                v.area = b;
                v.step(window);
                v.request(w, h, colors)
            });
            this
        },
        move |b, this, window, _| {
            drag::listen(window, this);
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
