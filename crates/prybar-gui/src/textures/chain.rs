//! Decodes a base texture into the mip chain the 3D view uploads.

use prybar_service::texture::{MaterialReport, Sources};
use prybar_vtf::{Image, Vtf};

use super::{Info, Slot};

/// Largest side sent to the GPU: the preview never fills more than a few
/// hundred pixels with one texture, and 4096 maps would cost 64 MB each.
pub const GPU_SIDE: u32 = 1024;

pub fn slot(report: MaterialReport, sources: &Sources) -> (Slot, Vec<Image>) {
    let mut slot = Slot {
        report,
        info: None,
        error: None,
    };
    let Some(base) = slot.report.base_texture.clone() else {
        slot.error = Some(match &slot.report.vmt {
            Some(_) => "The material has no $basetexture".into(),
            None => "Material file not found".into(),
        });
        return (slot, Vec::new());
    };
    match read(sources, &base) {
        Ok((info, vtf)) => {
            slot.info = Some(info);
            let levels = levels(&vtf, slot.report.cutout);
            (slot, levels)
        }
        Err(e) => {
            slot.error = Some(e);
            (slot, Vec::new())
        }
    }
}

fn read(sources: &Sources, base: &str) -> Result<(Info, Vtf), String> {
    let path = format!("materials/{base}.vtf");
    let found = sources.read(&path).map_err(|e| e.to_string())?;
    let found = found.ok_or_else(|| format!("{path} not found"))?;
    let vtf = Vtf::parse(found.bytes).map_err(|e| format!("{path}: {e}"))?;
    let h = &vtf.header;
    let info = Info {
        width: h.width,
        height: h.height,
        format: vtf.format_name(),
        mips: h.mips.max(1),
        location: found.location,
    };
    Ok((info, vtf))
}

/// Every mip from the first that fits `GPU_SIDE` down to the smallest,
/// stopping at the first that fails. Opaque materials get alpha forced
/// to 255 so the shader's cutout test only bites where the VMT asks.
pub fn levels(vtf: &Vtf, cutout: bool) -> Vec<Image> {
    let h = &vtf.header;
    let mips = h.mips.max(1);
    let first = (0..mips)
        .find(|&m| (h.width.max(h.height) >> m) <= GPU_SIDE)
        .unwrap_or(mips - 1);
    let mut out: Vec<Image> = (first..mips).map_while(|m| vtf.decode(m).ok()).collect();
    if !cutout {
        out.iter_mut()
            .for_each(|i| i.rgba.iter_mut().skip(3).step_by(4).for_each(|a| *a = 255));
    }
    out
}
