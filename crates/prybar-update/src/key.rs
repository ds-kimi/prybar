/// The owner's minisign public key, committed as `keys/update.pub`. Until
/// the owner adds a real key the file holds only a comment and updates fall
/// back to a download link.
const KEY_FILE: &str = include_str!("../keys/update.pub");

/// The embedded public key, if one has been committed.
pub fn embedded_key() -> Option<&'static str> {
    let usable = KEY_FILE
        .lines()
        .any(|l| l.starts_with("untrusted comment:"));
    usable.then_some(KEY_FILE)
}
