# prybar CLI

```
prybar [--json] [--quiet] <command> [args]
```

| Command | What it does |
|---|---|
| `inspect <model>` | Version, bones, meshes, materials, sequences of one model |
| `decompile <inputs>... -o <dir> [-r] [--split-qc]` | QC + SMD + VTA + VRD (+ GoldSrc BMP textures), one subfolder per model, models in parallel |
| `list <package> [--filter <wildcard>] [--limit <n>]` | Entries of a VPK, GMA, APK or HFS |
| `unpack <package> -o <dir>` | Extract every entry |
| `pack <dir> -o <out.vpk\|out.gma> [--vpk-version 1\|2] [--chunk-size <bytes>]` | Build a VPK or GMA |
| `compile <qc> -g <game>` | studiomdl of the configured game; JSON `outputs` lists the files written (.mdl, .vvd, .vtx, .phy, .ani, .cvvd) |
| `view <mdl> -g <game>` | hlmv of the configured game, detached |
| `publish <spec.json> [--dry-run] [--game <game>]` / `publish --list <game>` / `publish --status <game>` / `publish --games` | Steam Workshop create/update (needs `--features workshop`). `<game>` is `garrysmod`, `tf2` or the app id; `--games` lists supported games, the installed ones and why others are unsupported ([workshop-games](../workshop-games.md)) |
| `workshop items <game> [-s <text>] [--sort <key>] [--reverse] [--visibility <v>] [--tag <t>]` | Your items, searched and sorted like the app's list; sort keys `subscribed`, `rated`, `favorited`, `updated` (default), `newest`, `title`; visibility `public`, `friendsonly`, `private`, `unlisted` |
| `workshop item <game> <id>` | One item: fields, stats, daily stats points, version history from the changelog page |
| `workshop preview <image>` | Preview check: size, 512x512 crop, refusal reason |
| `workshop bbcode <file\|->` | BBCode description check: length (max 7999), tags used, plain text |
| `texture <file.vtf> -o <out.png> [--level <n>]` / `texture <model> --material <name> -o <out.png>` | Decode a VTF to PNG; with `--material` it is found like the engine does (model tree, package, configured games) |
| `privacy [--on\|--off] [--clear-log]` | The desktop app's analytics choice and its log of sent data (the CLI itself sends nothing) |
| `games [--detect] [--save]` | Configured games; detect Steam installs |
| `version [--check]` | Version; `--check` gives `{current, latest, update_available}` (`latest` is `null` offline) |
| `self-update` | Download, verify (SHA256SUMS + minisign) and install the latest release; `{current, latest, action, detail}` with `action` `up_to_date`, `installed` or `manual` (command or link in `detail`) |

Every GUI feature and its CLI and MCP counterparts: [parity](../agents/parity.md).

## Model inputs

- a `.mdl` file, a folder (`-r` to recurse), or a wildcard (`models/*.mdl`);
- a model inside a package: `pak_dir.vpk:models/props/crate.mdl`;
- a folder or wildcard inside a package: `pak_dir.vpk:models/props`,
  `addon.gma:models/*.mdl` (one folder level; with `-r`, `*` also
  matches subfolders).

Companions (`.vvd`, `.dx90.vtx`/`.vtx`/`.dx80.vtx`/`.sw.vtx`, `.phy`,
`.ani`, GoldSrc `T.mdl` and `NN.mdl`) are read from the same place.
Vampire: Bloodlines (MDL 2531) keeps its vertices in the `.mdl` and needs
only the `.dx80.vtx` (or `.dx7_2bone.vtx`) for triangles; there is no `.vvd`. A
missing or broken companion becomes a `notes` entry; the model is still
decompiled.

## Output

`--json` prints exactly one JSON document on stdout; progress goes to
stderr. Errors in JSON mode: `{"error": {"message": "...", "code": N}}`.
A batch never stops on a bad model: it lands in `failures` with its own
code.

## Update notice

At most once a day (cached), a background check may print one line on
stderr at the end of a command:
`prybar 0.3.0 is available (you have 0.2.1): prybar self-update`.
stdout and `--json` output never change. The check is skipped with
`--quiet`, when stderr is not a terminal, when `CI` is set, or with
`PRYBAR_NO_UPDATE_CHECK=1`. It sends nothing but a plain GitHub API
request. Installed by Homebrew, `cargo install` or a package manager,
`self-update` prints that tool's command instead of replacing files;
an unsigned release gives the download link only.

## Exit codes (stable)

| Code | Meaning |
|---|---|
| 0 | success |
| 1 | internal error |
| 2 | usage error (bad arguments, unknown game, missing tool path, read-only format) |
| 3 | input not found |
| 4 | invalid input (malformed model, package or Workshop item) |
| 5 | I/O error (unreadable input, unwritable output) |
| 6 | external tool failed (studiomdl, hlmv, Steam) |
| 7 | batch partly failed |

When nothing was decompiled the run exits with the failures' shared
code (4 if they differ); 7 means some models were written.
