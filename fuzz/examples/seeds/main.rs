//! Regenerates the committed seed corpus from our own writers and the
//! synthetic test builders, so seeds never contain game assets.
//! Run from `fuzz/`: `cargo run --example seeds`.

#[path = "../../../crates/prybar-hfs/tests/support/mod.rs"]
mod hfs_support;
#[path = "../../../crates/prybar-mdl/tests/common/mod.rs"]
mod mdl_common;
#[path = "../../../crates/prybar-mdl/tests/anim/fixture/mod.rs"]
mod anim_fixture;
#[path = "../../../crates/prybar-mdl/tests/flex/fixture.rs"]
mod flex_fixture;
mod packs;
#[path = "../../../crates/prybar-mdl/tests/phy/fixture.rs"]
mod phy_fixture;

use std::path::Path;

fn save(format: &str, name: &str, data: &[u8]) {
    let dir = Path::new("corpus").join(format);
    std::fs::create_dir_all(&dir).expect("create corpus dir");
    std::fs::write(dir.join(name), data).expect("write seed");
}

fn main() {
    for (name, data) in packs::vpks() {
        save("vpk", &name, &data);
    }
    save("gma", "addon.gma", &packs::gma());
    save("apk", "one.apk", &packs::apk());
    let hfs = hfs_support::build(&[("a.txt", b"hello"), ("d/b.bin", b"\x01\x02")]);
    save("hfs", "two.hfs", &hfs);
    save("mdl", "v49.mdl", &mdl_common::build());
    save("mdl", "anim.mdl", &anim_fixture::build(false));
    let mdl = anim_fixture::build(true);
    let mut split = (mdl.len() as u16).to_le_bytes().to_vec();
    split.extend(mdl.iter().chain(&anim_fixture::ani()));
    save("mdl_ani", "anim.bin", &split);
    save("mdl", "flex.mdl", &flex_fixture::build());
    save("phy", "one.phy", &phy_fixture::build());
    save("vvd", "v49.vvd", &mdl_common::geometry::vvd());
    save("vtx", "v49.vtx", &mdl_common::geometry::vtx());
}
