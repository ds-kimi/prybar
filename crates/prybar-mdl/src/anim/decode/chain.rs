use crate::read::offset;
use crate::Result;
use prybar_io::Reader;

/// The bone field of a record is a byte, so a chain longer than this is
/// a forged loop.
const MAX_RECORDS: usize = 256;

/// Maps each bone to its `mstudioanim_t` record; bones absent from the
/// chain keep their default pose.
pub fn records(r: &Reader, start: usize, bones: usize) -> Result<Vec<Option<usize>>> {
    let mut out = vec![None; bones];
    let mut at = start;
    for _ in 0..MAX_RECORDS {
        let bone = usize::from(r.read::<u8>(at)?);
        let next: i16 = r.read(at + 2)?;
        if let Some(slot) = out.get_mut(bone) {
            slot.get_or_insert(at);
        }
        if next == 0 {
            break;
        }
        at = offset(at, next.into())?;
    }
    Ok(out)
}
