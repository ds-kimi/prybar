use std::collections::BTreeMap;
use std::path::Path;
use walkdir::WalkDir;

/// Covers the tree's special cases: root files, no extension, empty
/// files, nested dirs, several dirs sharing an extension.
pub fn make_tree(root: &Path) {
    let files: [(&str, &[u8]); 7] = [
        ("root.txt", b"at the root"),
        ("noext", b"no extension"),
        ("cfg/.hidden", b"dotfile"),
        ("empty.dat", b""),
        ("materials/a/b/deep.vmt", b"\"LightmappedGeneric\" {}"),
        ("materials/c.vmt", b"other dir same ext"),
        ("models/big.mdl", &[7u8; 300]),
    ];
    for (rel, data) in files {
        let p = root.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, data).unwrap();
    }
}

pub fn snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
    WalkDir::new(root)
        .into_iter()
        .map(Result::unwrap)
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let rel = e.path().strip_prefix(root).unwrap();
            let rel = rel.to_str().unwrap().replace('\\', "/");
            (rel, std::fs::read(e.path()).unwrap())
        })
        .collect()
}
