---
name: prybar
description: Decompile, inspect, compile and package Source engine models and addons (MDL 04-53, QC/SMD/VTA/VRD, VPK, GMA, APK, HFS, studiomdl, Steam Workshop) with the prybar CLI or MCP server. Use when the user works with Garry's Mod or Source models, .mdl/.vpk/.gma files, or asks to decompile, recompile, unpack, pack or publish them.
---

# prybar

Source engine modding toolkit: CLI `prybar` and MCP server `prybar-mcp`. Prefer the MCP tools when they are registered (`docs/agents/mcp-config.md`), otherwise run the CLI with `--json`.

## When to use

- Turn a `.mdl` (+ `.vvd`, `.vtx`, `.phy`, `.ani`) into editable QC, SMD, VTA, VRD.
- List, extract or build `.vpk` / `.gma` packages; read `.apk` and `.hfs`.
- Compile a QC with the game's studiomdl; open a model in hlmv.
- Publish or update a Steam Workshop item.

## Workflow

1. `inspect` the model: version and which companions exist.
2. `decompile` to an output folder (one subfolder per model).
3. Edit the QC or SMD files.
4. `games --detect --save` once, then `compile -g <game>`.
5. `pack` the addon folder to `.gma` (needs `addon.json`) or `.vpk`.
6. `publish --dry-run`, then `publish`. Supported Workshop games: `garrysmod` (4000, addon folder or .gma) and `tf2` (440, map folder with a .bsp); pick with `--game`, list with `publish --games`. Descriptions are Steam BBCode.

Recipes with exact calls: `docs/agents/usecases.md`.

## Gotchas

- MCP paths are absolute; UNC and device paths are refused.
- Package inner paths: `pak_dir.vpk:models/props/crate.mdl`; folders and wildcards work too.
- Multi-archive VPK: use the `_dir.vpk`.
- A bad model in a batch lands in `failures`; the batch continues.
- `compile_model` failing is `success: false` with the log, not a tool error.
- MCP `config_set` cannot set `studiomdl` or `hlmv` paths on purpose; use `games --detect --save` or edit the config file.
- Never commit or redistribute game assets you decompile.

## JSON and exit codes

`--json` prints exactly one JSON document on stdout; progress is on stderr. Exit codes: 0 ok, 1 internal, 2 usage, 3 not found, 4 invalid input, 5 I/O, 6 external tool, 7 partial batch. MCP errors: `{error, kind, code}` with the same codes.

## Reference

- `docs/agents/errors.md`: every error, cause, fix
- `docs/agents/usecases.md`: task recipes
- `docs/agents/troubleshooting.md`: studiomdl, config, limits
- `docs/cli/README.md`, `docs/mcp/README.md`: full command and tool reference
