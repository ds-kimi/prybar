use crate::header::{fields::ATTACHMENTS, Header};
use crate::ir::Attachment;
use crate::read::Source;
use crate::read::{mat3x4, name_at, table};
use crate::Result;
use prybar_io::Cursor;

const STRIDE: usize = 92;

pub fn attachments(r: &Source, hdr: &Header) -> Result<Vec<Attachment>> {
    let (count, index) = hdr.span(ATTACHMENTS);
    table(r, 0, count, index, STRIDE)?
        .into_iter()
        .map(|at| attachment(r, at))
        .collect()
}

fn attachment(r: &Source, at: usize) -> Result<Attachment> {
    let mut c = Cursor::new(r.reader, at);
    let name_rel: i32 = c.read()?;
    Ok(Attachment {
        name: name_at(r, at, name_rel)?,
        flags: c.read()?,
        bone: c.read()?,
        local: mat3x4(&mut c)?,
    })
}
