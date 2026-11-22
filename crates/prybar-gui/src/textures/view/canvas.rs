//! The preview surface: paints the image where the pan puts it, keeps
//! drags tracking outside the region, and runs the glide.

use glam::Vec2;
use gpui::{
    canvas, point, px, size, Bounds, Canvas, Corners, DispatchPhase, Entity, MouseButton,
    MouseMoveEvent, MouseUpEvent, Pixels, Window,
};

use super::{Pan, TextureView};

#[derive(Debug, Clone, PartialEq)]
pub struct Drag {
    pub last: Vec2,
    /// Seconds since the press and cumulative travel, for the flick.
    pub samples: Vec<(f32, Vec2)>,
    pub start: std::time::Instant,
}

type Prepainted = (Entity<TextureView>, Option<Bounds<Pixels>>);

pub fn surface(v: &mut TextureView, this: Entity<TextureView>) -> Canvas<Prepainted> {
    let shown = v
        .shown
        .as_ref()
        .map(|s| (s.image.clone(), Vec2::new(s.width as f32, s.height as f32)));
    let drops = std::mem::take(&mut v.drops);
    let image = shown.as_ref().map(|s| s.0.clone());
    canvas(
        move |b, window, cx| {
            let at = this.update(cx, |v, _| {
                v.area = b;
                v.glide(window);
                shown.map(|(_, texels)| place(b, &v.pan, texels))
            });
            (this, at)
        },
        move |b, (this, at), window, _| {
            listen(window, this);
            for d in drops {
                let _ = window.drop_image(d);
            }
            if let (Some(img), Some(at)) = (image, at) {
                let _ = window.paint_image(b, at, Corners::default(), img, 0, false);
            }
        },
    )
}

/// Where the image lands: centered, shifted by the pan, at its scale.
pub fn place(b: Bounds<Pixels>, pan: &Pan, texels: Vec2) -> Bounds<Pixels> {
    let area = Vec2::new(f32::from(b.size.width), f32::from(b.size.height));
    let shown = texels * pan.scale(area, texels);
    let c = Vec2::new(f32::from(b.origin.x), f32::from(b.origin.y)) + area / 2.0 + pan.offset;
    let o = c - shown / 2.0;
    Bounds::new(point(px(o.x), px(o.y)), size(px(shown.x), px(shown.y)))
}

/// Window-wide so a drag that leaves the region keeps going and ends.
fn listen(window: &mut Window, this: Entity<TextureView>) {
    let moves = this.clone();
    window.on_mouse_event(move |e: &MouseMoveEvent, phase, _, cx| {
        if phase == DispatchPhase::Bubble && e.pressed_button == Some(MouseButton::Left) {
            let p = Vec2::new(f32::from(e.position.x), f32::from(e.position.y));
            moves.update(cx, |v, cx| v.drag_to(p, cx));
        }
    });
    window.on_mouse_event(move |e: &MouseUpEvent, phase, _, cx| {
        if phase == DispatchPhase::Bubble && e.button == MouseButton::Left {
            this.update(cx, |v, cx| v.let_go(cx));
        }
    });
}

impl TextureView {
    /// One glide step per frame while it moves, timed so 60 and 120 fps
    /// feel the same.
    fn glide(&mut self, window: &mut Window) {
        let now = std::time::Instant::now();
        let dt = self.tick.map_or(0.0, |t| (now - t).as_secs_f32());
        if self.drag.is_none() && self.pan.step(dt) {
            self.tick = Some(now);
            window.request_animation_frame();
        } else {
            self.tick = None;
        }
    }
}
