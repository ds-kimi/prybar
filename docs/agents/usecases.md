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

MCP: `decompile_model {"inputs": ["/abs/crate.mdl"], "out": "/abs/work"}`, then `compile_model {"game": "garrysmod", "qc": "/abs/work/crate/crate.qc"}`. Expect counts and `failures: []`, then `success: true`. The compiled model lands in the game's `models` folder; `outputs` lists every file written.

## Fix a broken model

1. `inspect` it; a `notes` entry or a code 4 error names the broken part.
2. Decompile with all companions in the same folder.
3. Fix the QC or SMD, then compile. If compile returns `success: false`, read the log (`troubleshooting.md`).

## Find and decompile models inside a package

Output mirrors the tree under the ref's prefix: `pak01_dir.vpk:models/props` gives `out/crate/`, `out/wood/box/`... (a whole package, `pak01_dir.vpk:`, gives `out/models/props/crate/`). `--flat` (MCP `flat: true`) gives `out/<name>/` per model.

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

`spec.json`: `{"app_id": 4000, "title": "My addon", "description": "...", "visibility": "private", "content": "/abs/addon_src", "preview": "/abs/preview.png", "change_note": "first upload"}`. `content` is an addon folder (packed for you; dry run lists every `addon.json` problem and refused file) or a `.gma`. Empty `tags` take `Addon` + type + tags from the folder's `addon.json`. The preview is cropped and resized to 512x512 (GIFs must already be under 1 MB). Add `"item_id": 123` to update an existing item. Release builds include Steam support; Steam must be running and logged in: `prybar --json publish --status 4000` says `ready`, `not_running`, `not_logged_in`, `not_owned` or `disabled`. `prybar --json publish --list 4000` lists your items. Games: `prybar --json publish --games` lists the supported ones (Garry's Mod 4000, Team Fortress 2 maps 440), which are installed here, and why L4D2, Portal 2 and others are not supported; `--game tf2` (or `--game 440`) overrides the spec's `app_id`. A TF2 item's `content` is a folder with at least one `.bsp` (in it or in `maps/`).

MCP: `unpack_package`, `pack_package {"src": ..., "out": "/abs/addon_new.gma"}`, `publish_workshop {"spec": "/abs/spec.json", "dry_run": true}` (add `"game": "tf2"` to target another game; `{"list_games": true}` lists the games, `{"game": "garrysmod"}` your items). A real publish also needs `"confirm": true`: ask the user first.

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

Check `failures` in the report. Exit 0 means all done, 7 some failed, otherwise the shared failure code (see `errors.md`). The output mirrors each input's folder tree.

## Decompile a model with its shared animations

Characters often pull their sequences from other models through `$includemodel` (`models/humans/male_shared.mdl`, `*_gestures.mdl`). They are decompiled with the model by default, once per run:

```sh
prybar decompile "hl2_misc_dir.vpk:models/humans/group01/male_01.mdl" -o out --json
```

Included models carry `included_from`; ones that cannot be found are listed in `warnings`. MCP: `decompile_model` does the same (`with_includes: false` to skip).

## Update my addon's description with a changelog

```sh
prybar --json workshop items garrysmod --search "crate"        # find the item id
prybar --json workshop item garrysmod 123456789                 # current description + history
# write the new description in desc.txt (Steam BBCode)
prybar --json workshop bbcode desc.txt                          # too_long must be false
```

Put the text in a spec `{"app_id": 4000, "item_id": 123456789, "description": "...", "change_note": "v1.2: fixed collisions"}` (fields left out keep their Steam value), then `prybar publish spec.json --dry-run` and `prybar publish spec.json`.

MCP: `workshop_items {"game": "garrysmod", "search": "crate"}`, `workshop_item {"game": "garrysmod", "item_id": 123456789}`, `check_bbcode {"text": "[h1]v1.2[/h1][list][*]fixed collisions[/list]"}`, then `publish_workshop {"spec": "/abs/spec.json", "dry_run": true}` and, after the user agrees, `{"spec": "/abs/spec.json", "confirm": true}`.

## Publish a new GMod addon from a folder

```sh
prybar --json publish --status garrysmod                        # state must be "ready"
prybar --json workshop preview /abs/preview.png                 # ok must be true
prybar publish spec.json --dry-run
prybar publish spec.json
```

`spec.json`: `{"app_id": 4000, "title": "Crate pack", "description": "[b]Crates[/b] for builds", "visibility": "private", "content": "/abs/crate_addon", "preview": "/abs/preview.png", "change_note": "first upload"}`. No `item_id` creates the item; the report gives its id. Update content later with the same spec plus `item_id`.

MCP: `workshop_status {"game": "garrysmod"}`, `check_preview {"image": "/abs/preview.png"}`, `publish_workshop {"spec": "/abs/spec.json", "dry_run": true}`, then with `"confirm": true`.

## Most subscribed items, or only private ones

```sh
prybar --json workshop items garrysmod --sort subscribed
prybar --json workshop items garrysmod --visibility private --tag weapon
```

MCP: `workshop_items {"game": "garrysmod", "sort": "subscribed"}`. Sorts: `subscribed`, `rated`, `favorited`, `updated`, `newest`, `title`; `reverse` flips the order.

## See a model's texture

```sh
prybar --json inspect crate.mdl                                 # material names
prybar --json texture crate.mdl --material models/props/crate01 -o crate01.png
prybar --json texture materials/models/props/wood.vtf -o wood.png --level 1
```

MCP: `decode_texture {"input": "/abs/crate.mdl", "material": "models/props/crate01", "out": "/abs/crate01.png"}`.

## Turn off the app's analytics, or see what was sent

```sh
prybar --json privacy                 # on, blocked_by, log of every payload
prybar --json privacy --off           # also drops anything queued
prybar privacy --clear-log
```

MCP: `privacy {}` to read; `privacy {"set": false, "confirm": true}` after the user asks. `DO_NOT_TRACK=1` or `PRYBAR_TELEMETRY=0` block sending whatever the choice says.

## File a bug found while working

See `reporting-bugs.md`: ask, check `gh auth status`, search duplicates, draft redacted, ask again, `gh issue create`.
