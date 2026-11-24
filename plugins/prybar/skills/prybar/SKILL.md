---
name: prybar
description: Decompile, inspect, compile and package Source engine models and addons (MDL 04-53, QC/SMD/VTA/VRD, VPK, GMA, APK, HFS, studiomdl, Steam Workshop) with the prybar CLI or MCP server. Use when the user works with Garry's Mod or Source models, .mdl/.vpk/.gma files, or asks to decompile, recompile, unpack, pack or publish them.
---

# prybar

Source engine modding toolkit: CLI `prybar` and MCP server `prybar-mcp`. Prefer the MCP tools when they are registered (`docs/agents/mcp-config.md`), otherwise run the CLI with `--json`.

## When to use

- Turn a `.mdl` (+ `.vvd`, `.vtx`, `.phy`, `.ani`) into editable QC, SMD, VTA, VRD. Vampire: Bloodlines (MDL 2531) needs only the `.dx80.vtx` next to it.
- List, extract or build `.vpk` / `.gma` packages; read `.apk` and `.hfs`.
- Compile a QC with the game's studiomdl; open a model in hlmv.
- Publish or update a Steam Workshop item; list, search and sort your items, read an item's stats and version history.
- Check a preview image or a BBCode description before upload.
- Decode a VTF texture (file or a model's material) to PNG.
- Read or change the desktop app's analytics choice and see what it sent.

## Workflow

1. `inspect` the model: version and which companions exist.
2. `decompile` to an output folder (one subfolder per model, mirroring the source tree; `$includemodel` models come along, see `included_from` and `warnings`; `--flat`, `--no-includes`).
3. Edit the QC or SMD files.
4. `games --detect --save` once, then `compile -g <game>` on a .qc, or on the whole output folder with `-r` (per-QC report, exit 7 if some failed).
5. `pack` the addon folder to `.gma` (needs `addon.json`) or `.vpk`.
6. `publish --dry-run`, then `publish`. Supported Workshop games: `garrysmod` (4000, addon folder or .gma) and `tf2` (440, map folder with a .bsp); pick with `--game`, list with `publish --games`. Descriptions are Steam BBCode.

Workshop helpers: `workshop items <game> [--search --sort --reverse --visibility --tag]`, `workshop item <game> <id>`, `workshop preview <image>`, `workshop bbcode <file|->` (MCP `workshop_items`, `workshop_item`, `check_preview`, `check_bbcode`, `workshop_status`). Textures: `texture <vtf> -o out.png` or `texture <model> --material <name> -o out.png` (MCP `decode_texture`). Privacy: `privacy [--on|--off] [--clear-log]` (MCP `privacy`, `confirm: true` to change).

Recipes with exact calls: `docs/agents/usecases.md`. GUI/CLI/MCP feature table: `docs/agents/parity.md`.

## Gotchas

- MCP paths are absolute; UNC and device paths are refused.
- Package inner paths: `pak_dir.vpk:models/props/crate.mdl`; folders and wildcards work too.
- Multi-archive VPK: use the `_dir.vpk`.
- A bad model in a batch lands in `failures`; the batch continues.
- `compile_model` failing is `success: false` with the log, not a tool error. On success `outputs` lists the written files. Batch: `inputs` (files, folders, `*.qc`) + `recursive`, items with `status` and `errors`, no logs.
- MCP `config_set` cannot set `studiomdl` or `hlmv` paths on purpose; use `games --detect --save` or edit the config file.
- Descriptions are Steam BBCode, at most 7999 characters; run `check_bbcode` first. Visibility: `public`, `friendsonly`, `private`, `unlisted`.
- Writes need `confirm: true` in MCP (`publish_workshop` without `dry_run`, `privacy` changes): ask the user first.
- Never commit or redistribute game assets you decompile.

## Reporting a prybar bug

Wrong output, a crash, exit 1 or an MCP error not caused by the input: stop and ask "Want me to open an issue on ds-kimi/prybar?". Continue only on an explicit yes. Check `gh auth status` (else give the prefilled text and the new-issue URL), search duplicates with `gh issue list --search`, draft the bug form fields with redacted paths and no game files, show the exact title, body and labels, ask again, and run `gh issue create` only after the second explicit yes. One issue per bug. Details: `docs/agents/reporting-bugs.md`.

## JSON and exit codes

`--json` prints exactly one JSON document on stdout; progress is on stderr. Exit codes: 0 ok, 1 internal, 2 usage, 3 not found, 4 invalid input, 5 I/O, 6 external tool, 7 partial batch. MCP errors: `{error, kind, code}` with the same codes.

## Reference

- `docs/agents/errors.md`: every error, cause, fix
- `docs/agents/usecases.md`: task recipes
- `docs/agents/troubleshooting.md`: studiomdl, config, limits
- `docs/agents/reporting-bugs.md`: how to file a bug (two confirmations)
- `docs/cli/README.md`, `docs/mcp/README.md`: full command and tool reference
