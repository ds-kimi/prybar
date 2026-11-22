//! Tokens of the Prybar design. Neutral surfaces carry the UI, a strong
//! neutral marks primary actions, and rust is the secondary accent for
//! the current place, selection, focus and progress only. Both themes are
//! drawn on purpose, not inverted. Measured contrast (WCAG):
//! dark: text 15.9, muted 8.3, rust 6.8 and danger 7.3 on bg;
//! light: text 16.0, muted 7.2, rust 5.4 and danger 5.5 on bg.
//!
//! Scales, by role: type 12 (meta) / 14 (body) / 16 (screen title);
//! spacing 4 / 8 / 12 / 16 / 24; radius 4 for small marks, 6 for
//! controls, 8 for panels and floating layers; shadows only on floating
//! layers (palette, toasts).

use gpui::{rgb, Rgba};

#[derive(Clone, Copy)]
pub struct Tokens {
    pub bg: Rgba,
    /// Panels and the nav bar sit one step above the window.
    pub surface: Rgba,
    /// Floating layers: palette, toasts.
    pub raised: Rgba,
    pub hover: Rgba,
    pub active: Rgba,
    pub border: Rgba,
    pub text: Rgba,
    pub muted: Rgba,
    pub rust: Rgba,
    pub primary: Rgba,
    pub on_primary: Rgba,
    pub danger: Rgba,
}

/// Near-black graphite, the default.
fn dark() -> Tokens {
    Tokens {
        bg: rgb(0x131417),
        surface: rgb(0x1a1c20),
        raised: rgb(0x212429),
        hover: rgb(0x25282d),
        active: rgb(0x2e3238),
        border: rgb(0x2c2f35),
        text: rgb(0xeceef1),
        muted: rgb(0xa8aeb7),
        rust: rgb(0xe8834f),
        primary: rgb(0xeceef1),
        on_primary: rgb(0x131417),
        danger: rgb(0xf2837a),
    }
}

/// Warm off-white; rust darkens so it keeps its contrast on paper.
fn light() -> Tokens {
    Tokens {
        bg: rgb(0xf7f5f1),
        surface: rgb(0xffffff),
        raised: rgb(0xffffff),
        hover: rgb(0xeeebe5),
        active: rgb(0xe4e0d8),
        border: rgb(0xdedad2),
        text: rgb(0x1b1a18),
        muted: rgb(0x55514b),
        rust: rgb(0xa9461a),
        primary: rgb(0x1b1a18),
        on_primary: rgb(0xf7f5f1),
        danger: rgb(0xb3362c),
    }
}

pub fn tokens(light_theme: bool) -> Tokens {
    if light_theme {
        light()
    } else {
        dark()
    }
}
