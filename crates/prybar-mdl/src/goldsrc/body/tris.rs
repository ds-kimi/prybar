use crate::read::{offset, Source};
use crate::Result;

pub type Corner = [i16; 4];

/// Decodes the triangle command list: a signed count (negative = fan,
/// positive = strip, 0 = end) then that many corners. Windings follow
/// the order studiomdl expects when it rebuilds strips from an SMD.
pub fn triangles(r: &Source, index: i32) -> Result<Vec<[Corner; 3]>> {
    let mut at = offset(0, index)?;
    let mut out = Vec::new();
    loop {
        let count: i16 = r.read(at)?;
        if count == 0 {
            return Ok(out);
        }
        let n = usize::from(count.unsigned_abs());
        let raw = r.array::<i16>(at + 2, n * 4, n * 4)?;
        let corners: Vec<Corner> = raw.as_chunks::<4>().0.to_vec();
        r.charge(n * 8)?;
        at += 2 + n * 8;
        split(&corners, count < 0, &mut out);
    }
}

fn split(c: &[Corner], fan: bool, out: &mut Vec<[Corner; 3]>) {
    for k in 0..c.len().saturating_sub(2) {
        out.push(match (fan, k % 2) {
            (true, _) => [c[0], c[k + 2], c[k + 1]],
            (false, 0) => [c[k], c[k + 2], c[k + 1]],
            _ => [c[k], c[k + 1], c[k + 2]],
        });
    }
}
