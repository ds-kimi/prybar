# Use cases

Each recipe gives the CLI call and the MCP tool call. Add `--json` to the CLI for parseable output. MCP paths are absolute.

## Inspect an unknown .mdl

```sh
prybar inspect models/props/crate.mdl --json
```

MCP: `inspect_model {"model": "/abs/crate.mdl"}`. Returns version, name, bone/mesh/sequence/material counts and which companions (`.vvd`, `.vtx`, `.phy`, `.ani`) were found. For a model in a package use `pak01_dir.vpk:models/props/crate.mdl`.

## Decompile, edit the QC, recompile

```sh
prybar decompile crate.mdl -o work
# edit work/crate/crate.qc (and the SMD files)
prybar games --detect --save        # once
prybar compile work/crate/crate.qc -g garrysmod
```

MCP: `decompile_model {"inputs": ["/abs/crate.mdl"], "out": "/abs/work"}`, then `compile_model {"game": "garrysmod", "qc": "/abs/work/crate/crate.qc"}`. Expect counts and `failures: []`, then `success: true`. The compiled model lands in the game's `models` folder.

## Fix a broken model

1. `inspect` it; a `notes` entry or a code 4 error names the broken part.
2. Decompile with all companions in the same folder.
3. Fix the QC or SMD, then compile. If compile returns `success: false`, read the log (`troubleshooting.md`).

## Find and decompile models inside a package

Output keeps the inner path: `out/models/props/crate/crate.qc` (loose files give `out/<name>/<name>.qc`).

```sh
prybar list pak01_dir.vpk --filter "models/props/*.mdl" --limit 20
prybar decompile "pak01_dir.vpk:models/props" -o out -r
```

MCP: `list_package {"package": "/abs/pak01_dir.vpk", "filter": "models/props/*.mdl", "limit": 20}`, then `decompile_model` with `"inputs": ["/abs/pak01_dir.vpk:models/props"], "recursive": true`.

## Extract, change and pack a GMA, then publish

```sh
prybar unpack addon.gma -o addon_src
# edit files; keep addon.json
prybar pack addon_src -o addon_new.gma
prybar publish spec.json --dry-run
prybar publish spec.json
```

`spec.json`: `{"app_id": 4000, "title": "My addon", "description": "...", "tags": ["model"], "visibility": "private", "content": "/abs/addon_new.gma", "preview": "/abs/preview.jpg", "change_note": "first upload"}`. Add `"item_id": 123` to update an existing item. Publishing needs a build with `--features workshop` and a running Steam client.

MCP: `unpack_package`, `pack_package {"src": ..., "out": "/abs/addon_new.gma"}`, `publish_workshop {"spec": "/abs/spec.json", "dry_run": true}`.

## Pack a VPK

```sh
prybar pack content -o mypak_dir.vpk --vpk-version 2 --chunk-size 200000000
```

`--chunk-size` splits data into `mypak_000.vpk`, `mypak_001.vpk`...; the output name must end in `_dir.vpk`.

## Batch work

Folders, wildcards and many inputs run in parallel. A bad model never stops the batch:

```sh
prybar decompile "models/*.mdl" other_dir -o out -r --json
```

Check `failures` in the report. Exit 0 means all done, 7 some failed, otherwise the shared failure code (see `errors.md`).
