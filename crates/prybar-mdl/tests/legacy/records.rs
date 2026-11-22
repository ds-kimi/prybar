use super::builder::{put, put_str};

pub const BONE: usize = 600;
pub const SET: usize = 900;
pub const TEX: usize = 1000;
pub const PART: usize = 1100;
pub const ATT: usize = 1500;

pub fn records(buf: &mut [u8], model_name: usize) {
    put(buf, BONE, 1900 - BONE as i32);
    put(buf, BONE + 4, -1);
    put(buf, SET, 1910 - SET as i32);
    put(buf, SET + 4, 1);
    put(buf, SET + 8, 12);
    put(buf, SET + 12, 0);
    put(buf, SET + 16, 2);
    put(buf, TEX, 1920 - TEX as i32);
    put(buf, PART, 1930 - PART as i32);
    put(buf, PART + 4, 1);
    put(buf, PART + 8, 1);
    put(buf, PART + 12, 16);
    put_str(buf, PART + 16, "body_ref");
    put(buf, PART + 16 + model_name + 4, 9f32.to_bits() as i32);
    put(buf, ATT, 1940 - ATT as i32);
    put(buf, ATT + 8, 0);
    for (at, s) in [
        (1900, "root"),
        (1910, "default"),
        (1920, "skin0"),
        (1930, "body"),
        (1940, "muzzle"),
    ] {
        put_str(buf, at, s);
    }
}
