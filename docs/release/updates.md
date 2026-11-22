# Release updates

`prybar-update` lets the CLI, the MCP server and the GUI find a newer
release and install it. It only installs files it has verified twice:

1. `SHA256SUMS.minisig` must be a valid minisign signature of `SHA256SUMS`
   made with the key whose public half is embedded in the binary
   (`crates/prybar-update/keys/update.pub`).
2. The downloaded archive must match its line in `SHA256SUMS`.

If either check fails, nothing is installed. If a release has no signature,
or the binary was built without a public key, users get "update available"
with the release link instead of an automatic install.

## How the check works

- One GET to `api.github.com/repos/ds-kimi/prybar/releases/latest`, 5 s
  timeout, `If-None-Match` with the ETag cached in the user cache dir
  (`update-check.json`). Nothing else is sent.
- Drafts and prereleases are ignored. Versions compare as semver.
- Offline, rate limited or garbled answers count as "unknown": no error,
  no hang.
- The archive picked is `prybar-<tag>-<target>.zip` or `.tar.gz` for the
  platform the binary was built for.

## How the install works

- A binary in a writable folder is replaced in place (`self_replace` for the
  running executable, which also handles Windows).
- On macOS `Prybar.app` is swapped as a whole bundle, so its signature and
  notarization stay intact.
- Installs owned by a package manager are left alone and the user gets the
  command: Homebrew (`brew upgrade prybar`), `cargo install`, Nix, Snap,
  `/usr/bin`. A folder that is not writable gets the release link.

## Owner setup (once)

The worker that wrote this never created, saw or stored the real key. Do
this on your own machine:

1. Install minisign (`brew install minisign`).
2. Generate the key pair, with a password:
   `minisign -G -p minisign.pub -s minisign.key`
3. Copy `minisign.pub` over `crates/prybar-update/keys/update.pub` and
   commit it. This is the public half; it is safe to publish.
4. In GitHub, **Settings > Environments > release**, add the secrets
   `UPDATE_SIGNING_KEY` (the whole content of `minisign.key`) and
   `UPDATE_SIGNING_KEY_PASSWORD`.
5. Keep `minisign.key` and its password offline (password manager). Do not
   commit it.

From the next `v*` tag, the `publish` job signs `SHA256SUMS` and uploads
`SHA256SUMS.minisig` next to it. Without the secret the job prints a notice
and the release ships unsigned, as before.

## Rotating the key

Generate a new pair, commit the new `update.pub` and replace the secrets.
Binaries released before the change only trust the old key, so users of
those versions get the download link for one release, then auto-update
again.

## Check a release by hand

```sh
minisign -Vm SHA256SUMS -p crates/prybar-update/keys/update.pub
shasum -a 256 -c SHA256SUMS --ignore-missing
```
