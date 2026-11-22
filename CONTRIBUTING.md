# Contributing to prybar

## Setup

1. Install Rust with [rustup](https://rustup.rs). The workspace builds on
   stable; fuzzing needs nightly:
   ```sh
   rustup toolchain install stable nightly
   rustup component add rustfmt clippy
   ```
2. Clone and build:
   ```sh
   git clone <repo> prybar && cd prybar
   cargo build --workspace
   ```
3. Optional tools:
   ```sh
   cargo install cargo-fuzz cargo-deny
   ```
   `actionlint` (workflow linter): binary from
   <https://github.com/rhysd/actionlint/releases>, put it on `PATH`.

## Running the CLI

```sh
cargo run -p prybar-cli -- --help
cargo run -p prybar-cli -- --json list path/to/pak_dir.vpk --limit 10
```

Commands, model inputs and exit codes: `docs/cli/README.md`. The CLI and
the MCP server share `crates/prybar-service`; put new behavior there, not
in a front end. `PRYBAR_CONFIG` points both at another config file (the
CLI tests use it so they never touch the real one).

## Running the MCP server

```sh
cargo build -p prybar-mcp
./target/debug/prybar-mcp < scripts/mcp/handshake.jsonl
```

Prints the `initialize` and `tools/list` answers (JSON-RPC, one per
line). Tools, path rules and client setup: `docs/mcp/README.md`. Tests:
`cargo test -p prybar-mcp` (in-process rmcp client for every tool, plus
the same handshake against the real binary).

## Checks (run all before every commit)

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
sh scripts/check-limits/check-limits.sh     # Windows: scripts/check-limits/check-limits.ps1
cargo deny check advisories licenses bans sources
actionlint
```

`check-limits` enforces the repo rules rustfmt and clippy cannot: at most
4 fns per `.rs` file, 30 lines per fn, 10 `.rs` files per folder, no
trailing spaces.

## Tests

- Unit tests sit next to the code; integration tests in
  `crates/<crate>/tests/`.
- Property tests (`proptest`) check every writer round-trips
  (`tests/proptest.rs`) and that mutated inputs never panic
  (`tests/mutate.rs`).
- Real-game tests read Garry's Mod files from `PRYBAR_GMOD_DIR`. Unset, the
  install is found through Steam on every OS (`prybar-tools` `paths/`):
  `PRYBAR_STEAM_DIR`, the Windows registry `SteamPath`, then the usual roots
  (`~/.steam/steam`, `~/.local/share/Steam`, Flatpak, macOS
  `~/Library/Application Support/Steam`) and every library in
  `steamapps/libraryfolders.vdf`. They skip when nothing is found. Never commit game assets.
- `prybar-tools` `tests/compile/` compiles a synthetic one-triangle QC with
  the detected GMod `studiomdl.exe` into a scratch game dir (Windows only;
  elsewhere it skips, the Wine runner is not exercised by tests).
- `prybar-workshop` links the Steam SDK only with `--features workshop`
  (`cargo test -p prybar-workshop --features workshop`, `cargo deny
  --features workshop check`). Tests only run dry runs; a real publish
  needs Steam running and logged in, and `steam_api64.dll` (Windows) /
  `libsteam_api.so` / `libsteam_api.dylib` next to the binary.
- The game config is `<user config dir>/prybar/config.toml` (`%APPDATA%`,
  `~/Library/Application Support`, `$XDG_CONFIG_HOME` or `~/.config`);
  `PRYBAR_CONFIG` points it at another file. On Linux/macOS studiomdl and
  hlmv run through `PRYBAR_WINE` (default `wine` on PATH).
- `prybar-mdl` `tests/gmod_anim/` decodes every animation of the GMod VPK
  (with the model's `.ani` when present); run it with `--release`, it is
  slow in debug builds.
- QC golden tests (`prybar-export` `tests/qc/`) use `insta` snapshots in
  `tests/qc/snapshots/`. After an intended QC change, regenerate them with
  `INSTA_UPDATE=always cargo test -p prybar-export --test qc` and review
  the diff before committing.

## Fuzzing

Targets live in `fuzz/fuzz_targets/<format>.rs`, seeds in
`fuzz/corpus/<format>/`. `fuzz/` is its own workspace.

```sh
cd fuzz
cargo +nightly fuzz list
mkdir -p work/vpk
cargo +nightly fuzz run -O vpk work/vpk corpus/vpk -- -max_total_time=60
```

Pass a scratch folder first so new inputs land there and the committed
corpus stays small. Seeds are regenerated from our own writers with
`cargo run --example seeds` (in `fuzz/`). The `mdl_ani` target reads a
model plus its `.ani` file from one input: the first two bytes (u16 LE)
give the MDL length, the rest is the `.ani`.

On Windows (MSVC), the fuzzer needs the ASan runtime
`clang_rt.asan_dynamic-x86_64.dll` on `PATH`. It ships with Visual Studio's
"C++ AddressSanitizer" component, e.g.
`C:\Program Files\Microsoft Visual Studio\<ver>\<edition>\VC\Tools\MSVC\<msvc>\bin\Hostx64\x64`.

## Benchmarks

```sh
cargo bench -p prybar-vpk -p prybar-gma -p prybar-mdl -p prybar-export
```

## Round-trip harness

Decompiles a sample of real models from the local Garry's Mod install
(GMod, HL2 and CS:S VPKs), rebuilds each with GMod's `studiomdl` in a
temp game dir, re-parses the result and compares bones, vertex count
(2% tolerance), sequences and frames, flexes and used materials. Needs
Windows (or `PRYBAR_WINE` on Linux/macOS) and a GMod install found via
`PRYBAR_GMOD_DIR` or Steam; without one it prints `skip:` and exits 0.

```sh
scripts/roundtrip/roundtrip.sh 25            # 25 models per package
pwsh scripts/roundtrip/roundtrip.ps1 -Count 25
```

Writes `target/roundtrip/roundtrip.{json,md}`; exits 1 below 90% pass.
Not in CI: it needs game files.

## Structure

- One crate per format under `crates/`; `src/` split into nested module
  folders by domain, one concept per file, at most ~8 files per folder.
- Tests in `crates/<crate>/tests/`, benches in `crates/<crate>/benches/`,
  fixtures in `tests/fixtures/<format>/`, scripts in `scripts/<purpose>/`,
  docs in `docs/<topic>/`.
- All input files are untrusted: bounds-check every offset and count,
  never panic. `unsafe` is forbidden workspace-wide; see
  `docs/UNSAFE-INVENTORY.md`.
- Code and comments in English; comments explain why, not what.

## Commits

Conventional commits (`feat(vpk): ...`, `fix(mdl): ...`).

## CI

`.github/workflows/`: `ci.yml` (fmt, clippy, test, check-limits on
Windows/Linux/macOS + cargo-deny), `fuzz.yml` (60 s smoke on PRs touching
parsers, 30 min nightly), `release.yml` (tag `v*`: `prybar` and
`prybar-mcp` for Windows, Linux and macOS with SHA-256 checksums).
