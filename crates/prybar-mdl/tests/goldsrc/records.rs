use super::builder::{float, put, text};
use std::f32::consts::FRAC_PI_2;

/// Bone 1 sits one unit along the root's X, which the root's yaw turns
/// into model-space Y.
pub fn skeleton(b: &mut [u8], bones: usize, att: usize, hitbox: usize) {
    for (i, name) in ["root", "child"].iter().enumerate() {
        let at = bones + i * 112;
        text(b, at, name);
        put(b, at + 32, i as i32 - 1);
        let scales = at + 32 + 14 * 4;
        float(b, scales, 0.1);
    }
    float(b, bones + 32 + 13 * 4, FRAC_PI_2);
    float(b, bones + 112 + 32 + 8 * 4, 1.0);
    text(b, att, "muzzle");
    put(b, att + 36, 1);
    float(b, att + 40, 4.0);
    put(b, hitbox, 1);
    put(b, hitbox + 4, 3);
    float(b, hitbox + 20, 2.0);
}

/// 2x2 texture whose pixel indices are 0..4 and whose palette is grey.
pub fn texture(b: &mut [u8], tex: usize) {
    text(b, tex, "skin.bmp");
    put(b, tex + 68, 2);
    put(b, tex + 72, 2);
    put(b, tex + 76, (tex + 80) as i32);
    b[tex + 80..tex + 84].copy_from_slice(&[0, 1, 2, 3]);
    for (i, c) in b[tex + 84..tex + 84 + 768].iter_mut().enumerate() {
        *c = (i / 3) as u8;
    }
}

/// "idle": 2 frames at 30 fps, one event on frame 1, child X keyed 10/20.
pub fn sequence(b: &mut [u8], seq: usize, event: usize, anim: usize) {
    text(b, seq, "idle");
    float(b, seq + 32, 30.0);
    put(b, seq + 32 + 4, 1);
    put(b, seq + 32 + 16, 1);
    put(b, seq + 32 + 20, event as i32);
    put(b, seq + 32 + 24, 2);
    put(b, seq + 32 + 88, 1);
    put(b, seq + 32 + 92, anim as i32);
    put(b, event, 1);
    put(b, event + 4, 5001);
    text(b, event + 12, "10");
    b[anim + 12..anim + 14].copy_from_slice(&12u16.to_le_bytes());
    b[anim + 24..anim + 30].copy_from_slice(&[2, 2, 10, 0, 20, 0]);
}
