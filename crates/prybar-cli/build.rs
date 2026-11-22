// Embeds the app icon and version info into the Windows executable; other
// targets need nothing, so the work is gated on the target, not the host.
fn main() {
    println!("cargo:rerun-if-changed=../../assets/prybar.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        embed();
    }
}

#[cfg(windows)]
fn embed() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/prybar.ico");
    if let Err(err) = res.compile() {
        println!("cargo:warning=icon not embedded: {err}");
    }
}

#[cfg(not(windows))]
fn embed() {}
