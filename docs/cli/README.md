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
| `compile <qc> -g <game>` | studiomdl of the configured game |
| `view <mdl> -g <game>` | hlmv of the configured game, detached |
| `publish <spec.json> [--dry-run]` / `publish --list <app_id>` | Steam Workshop create/update (needs `--features workshop`) |
| `games [--detect] [--save]` | Configured games; detect Steam installs |

## Model inputs

- a `.mdl` file, a folder (`-r` to recurse), or a wildcard (`models/*.mdl`);
- a model inside a package: `pak_dir.vpk:models/props/crate.mdl`;
- a folder or wildcard inside a package: `pak_dir.vpk:models/props`,
  `addon.gma:models/*.mdl` (one folder level; with `-r`, `*` also
  matches subfolders).

Companions (`.vvd`, `.dx90.vtx`/`.vtx`/`.dx80.vtx`/`.sw.vtx`, `.phy`,
`.ani`, GoldSrc `T.mdl` and `NN.mdl`) are read from the same place. A
missing or broken companion becomes a `notes` entry; the model is still
decompiled.

## Output

`--json` prints exactly one JSON document on stdout; progress goes to
stderr. Errors in JSON mode: `{"error": {"message": "...", "code": N}}`.
A batch never stops on a bad model: it lands in `failures` with its own
code.

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
