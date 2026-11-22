use crate::read::{name_at, table, Source};
use crate::{Error, Result};

/// A zero index means the list is absent, whatever the count says.
pub fn floats(r: &Source, seq: usize, count: usize, index: i32) -> Result<Vec<f32>> {
    if index == 0 || count == 0 {
        return Ok(Vec::new());
    }
    let n = i32::try_from(count).map_err(|_| Error::Invalid {
        what: "list length",
        value: count as i64,
    })?;
    let starts = table(r, seq, n, index, 4)?;
    Ok(r.array(starts[0], count, count)?)
}

/// Pose keys hold one value per blend row and column.
pub fn pose_keys(r: &Source, seq: usize, group: [i32; 2], index: i32) -> Result<Vec<f32>> {
    let n = i64::from(group[0]) + i64::from(group[1]);
    let n = usize::try_from(n).map_err(|_| Error::Invalid {
        what: "blend group size",
        value: n,
    })?;
    floats(r, seq, n, index)
}

pub fn anim_indices(r: &Source, seq: usize, group: [i32; 2], index: i32) -> Result<Vec<i16>> {
    let n = group[0].checked_mul(group[1]).ok_or(Error::Invalid {
        what: "blend group size",
        value: group[0].into(),
    })?;
    if index == 0 || n == 0 {
        return Ok(Vec::new());
    }
    let starts = table(r, seq, n, index, 2)?;
    starts.into_iter().map(|at| Ok(r.read(at)?)).collect()
}

/// `mstudioactivitymodifier_t` is a name index; v52+ append an int.
pub fn activity_modifiers(
    r: &Source,
    seq: usize,
    (count, index): (i32, i32),
    version: i32,
) -> Result<Vec<String>> {
    if index == 0 {
        return Ok(Vec::new());
    }
    let stride = if version >= 52 { 8 } else { 4 };
    table(r, seq, count, index, stride)?
        .into_iter()
        .map(|at| name_at(r, at, r.read(at)?))
        .collect()
}
