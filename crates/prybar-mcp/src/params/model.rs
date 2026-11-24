use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct InspectArgs {
    /// Absolute path of one `.mdl` file, or `<absolute package path>:<inner path>` for a model inside a VPK/GMA/APK/HFS package.
    pub model: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DecompileArgs {
    /// Models to decompile. Each item is an absolute path to a `.mdl` file, a folder, a package (`.vpk`, `.gma`, `.apk`, `.hfs`), or `<package>:<inner path>`. `*` and `?` wildcards are allowed in the file name or inner path. At most 64 items.
    pub inputs: Vec<String>,
    /// Absolute output folder. Each model gets its own sub-folder holding the QC, SMD, VTA and VRD files, placed at the model's path relative to its input (folder, wildcard base or package prefix), so the source tree is mirrored.
    pub out: String,
    /// Also search sub-folders of folder or package inputs.
    #[serde(default)]
    pub recursive: bool,
    /// Write one `.qci` include per QC section instead of a single QC file.
    #[serde(default)]
    pub split_qc: bool,
    /// Put every model in `<out>/<model name>/` instead of mirroring the source tree. Same-named models then get a `_2`, `_3`... suffix and a warning.
    #[serde(default)]
    pub flat: bool,
    /// Also decompile the models named by `$includemodel` (shared `*_animations.mdl`, `*_gestures.mdl`...), once each; they are listed with `included_from`. Default true.
    #[serde(default = "yes")]
    pub with_includes: bool,
}

fn yes() -> bool {
    true
}
