use super::fields::STUDIOHDR2_INDEX;
use super::Header;
use crate::read::Source;
use crate::read::{name_at, offset};
use crate::Result;

/// `studiohdr2_t::sznameindex`: the full model path when it does not fit
/// the 64-byte name field of the main header.
const NAME_FIELD: usize = 20;
/// v53 moved that index into the main header, right after the checksum.
const V53_NAME_FIELD: usize = 12;

pub fn apply_long_name(r: &Source, hdr: &mut Header) -> Result<()> {
    let (base, field) = if hdr.version == 53 {
        (0, V53_NAME_FIELD)
    } else {
        let rel = hdr.int(STUDIOHDR2_INDEX);
        if rel <= 0 {
            return Ok(());
        }
        let base = offset(0, rel)?;
        (base, base + NAME_FIELD)
    };
    let name = name_at(r, base, r.read(field)?)?;
    if !name.is_empty() {
        hdr.name = name;
    }
    Ok(())
}
