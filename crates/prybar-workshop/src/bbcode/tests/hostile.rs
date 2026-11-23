use super::super::parse;
use super::{depth, text};

/// Deep nesting stops at the cap and huge input stays linear.
#[test]
fn hostile_input() {
    let deep = "[quote]".repeat(10_000) + "x";
    assert!(depth(&parse(&deep)) <= 1, "unclosed tags turn into text");
    let closed = "[b]".repeat(1000) + &"[/b]".repeat(1000);
    assert!(depth(&parse(&closed)) <= 25);
    let huge = "[".repeat(200_000) + &"[code]".repeat(50_000);
    let start = std::time::Instant::now();
    let _ = parse(&huge);
    assert!(start.elapsed().as_secs() < 5);
}

/// Random mixes of tag pieces and odd chars never panic, and text with
/// no tag in it comes back unchanged.
#[test]
fn fuzz_smoke() {
    let mut seed = 0x2545_f491_u64;
    for _ in 0..3000 {
        let mut s = String::new();
        for _ in 0..(seed % 40) {
            seed = shuffle(seed);
            s.push_str(PARTS[(seed % PARTS.len() as u64) as usize]);
        }
        let _ = parse(&s);
    }
    assert_eq!(parse("a [ b ] c"), vec![text("a [ b ] c")]);
}

/// xorshift: plain, seeded, so a failure repeats.
fn shuffle(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^ (x << 17)
}

/// Pieces the fuzz smoke test strings together from.
const PARTS: [&str; 21] = [
    "[",
    "]",
    "/",
    "=",
    "[b]",
    "[/b]",
    "[*]",
    "[list]",
    "[/list]",
    "[code]",
    "[/code]",
    "[url=",
    "é",
    "\n",
    "[table",
    " ",
    "[hr]",
    "[/",
    "x",
    "[noparse]",
    "🙂",
];
