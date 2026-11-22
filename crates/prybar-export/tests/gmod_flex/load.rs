use prybar_mdl::ModelIr;
use prybar_vpk::Vpk;

pub fn read(vpk: &Vpk, path: &str) -> Option<Vec<u8>> {
    let e = vpk.entries.iter().find(|e| e.path == path)?;
    Some(vpk.read(e).unwrap())
}

/// MDL with VVD/VTX geometry and the `.phy` when one exists.
pub fn model(vpk: &Vpk, stem: &str) -> Option<ModelIr> {
    let get = |ext: &str| read(vpk, &format!("{stem}{ext}"));
    let (mdl, vvd, vtx) = (get(".mdl")?, get(".vvd")?, get(".dx90.vtx")?);
    let mut ir = prybar_mdl::parse(&mdl).unwrap();
    let vvd = prybar_mdl::parse_vvd(&vvd).unwrap();
    let vtx = prybar_mdl::parse_vtx(&vtx).unwrap();
    prybar_mdl::load_geometry(&mut ir, &vvd, &vtx).unwrap();
    ir.physics = get(".phy")
        .filter(|p| !p.is_empty())
        .map(|p| prybar_mdl::parse_phy(&p).unwrap_or_else(|e| panic!("{stem}.phy: {e}")));
    Some(ir)
}

pub fn stems<'a>(vpk: &'a Vpk, ext: &'a str) -> impl Iterator<Item = &'a str> {
    vpk.entries
        .iter()
        .filter_map(move |e| e.path.strip_suffix(ext))
}
