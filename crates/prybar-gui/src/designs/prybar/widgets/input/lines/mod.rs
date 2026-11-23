//! How a field's text is cut into the rows it is drawn in. A long line
//! of a multi-line field breaks at a space to fit the width; the break
//! is only drawn, the text never gets a newline from it.

mod rows;
#[cfg(test)]
mod tests;
pub mod text;

use std::ops::Range;

use gpui::ShapedLine;

/// Rows of one line (offsets inside it) for `max` pixels, given the x
/// of each byte offset. A word wider than the row is cut between chars.
pub fn wrap_line(line: &str, max: f32, x: impl Fn(usize) -> f32) -> Vec<Range<usize>> {
    let mut rows = Vec::new();
    let (mut start, mut space) = (0, None);
    for (i, c) in line.char_indices() {
        // Spaces may hang past the edge, so a row never starts with one.
        if c != ' ' && i > start && x(i + c.len_utf8()) - x(start) > max {
            let cut = space.filter(|s| *s > start).unwrap_or(i);
            rows.push(start..cut);
            start = cut;
            space = None;
        }
        if c == ' ' {
            space = Some(i + 1);
        }
    }
    rows.push(start..line.len());
    rows
}

/// The row an offset is drawn on. Where a wrapped row ends the next one
/// starts, so that offset belongs to the next row, like in text editors.
pub fn row_of(rows: &[Range<usize>], at: usize) -> usize {
    rows.iter().rposition(|r| r.start <= at).unwrap_or(0)
}

/// The x of every glyph's byte offset, then the end, read once per line
/// so wrapping does not walk the glyphs for each char.
pub fn edges(shaped: &ShapedLine) -> Vec<(usize, f32)> {
    let mut xs: Vec<(usize, f32)> = shaped
        .runs
        .iter()
        .flat_map(|r| r.glyphs.iter())
        .map(|g| (g.index, f32::from(g.position.x)))
        .collect();
    xs.sort_by_key(|e| e.0);
    xs.push((shaped.len, f32::from(shaped.width)));
    xs
}

/// The x of byte `at` from [`edges`]: the first glyph at or after it.
pub fn x_at(xs: &[(usize, f32)], at: usize) -> f32 {
    let i = xs.partition_point(|e| e.0 < at);
    xs.get(i).or(xs.last()).map_or(0., |e| e.1)
}
