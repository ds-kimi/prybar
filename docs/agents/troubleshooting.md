# Troubleshooting

## studiomdl or hlmv not found (`cannot start ...`, `has no studiomdl path configured`)

1. `prybar games --json` shows the configured games and tool paths.
2. `prybar games --detect --save` adds Steam games and their tools. `PRYBAR_GMOD_DIR` and `PRYBAR_STEAM_DIR` override detection.
3. Still empty: edit the config file (`PRYBAR_CONFIG`, else `<user config dir>/prybar/config.toml`) and set `studiomdl = "..."` in the `[[games]]` entry. MCP `config_set` refuses this field on purpose.
4. On Linux and macOS studiomdl is a Windows binary: run it through Wine or Proton.

## Game dir problems

`game_dir` is the folder holding `gameinfo.txt` (for GMod, `.../GarrysMod/garrysmod`). A wrong folder makes studiomdl report missing files or fail at start. Game names are case-insensitive.

## Path refusals (`not an absolute local path`)

MCP tools take absolute paths only. UNC (`\\host\share`) and device paths (`\\.\`, `\\?\UNC\`) are refused because opening them leaks credentials. Copy the file to a local folder first.

## Compile fails (`success: false`)

Read the returned log. Frequent causes:

- A missing SMD: the QC is not next to its SMD files. Compile from the decompile output folder.
- Bone or vertex limits: studiomdl caps them; reduce in the SMD.
- A `$sequence` naming an animation that was not decompiled: the animation lives in an `.ani` or an include model. Decompile with all companions in the same folder.

## Decompile output looks wrong

`notes` in the report lists companions that were missing or broken (`.vvd`, `.vtx`, `.phy`, `.ani`). Put all companions next to the `.mdl`, or reference the package folder (`pak_dir.vpk:models/props`). Known small differences: `docs/roundtrip-report.md`.

## Limits

- 64 inputs per `decompile_model`; the reply shows the first 50 results and failures, the rest is on disk.
- `list_package` returns 200 entries by default and at most 5000; `total` counts every match.
- Spec and config files over 1 MiB are refused.
- HFS `.comp` entries are listed, not extracted.
- VPK output: versions 1 and 2 only. GMA output needs `addon.json` and passes the GMod file whitelist.

## Workshop

`built without the workshop feature`: use a release build or rebuild with `--features workshop`. `Library not loaded: libsteam_api`: the Steam API library must sit next to the binary (release archives ship it). `prybar publish --status 4000` names what is missing. `steam: ...`: Steam must be running and signed in. Run with `--dry-run` first: it validates without contacting Steam. Visibility defaults to `private`.

## Updates

- `prybar version --check --json`: `latest: null` means GitHub was not
  reachable (offline, proxy, rate limit); it is never an error.
- `self-update` exits 6 when the check fails, 4 when the download does
  not match SHA256SUMS or its signature (nothing is installed), 5 when
  the install dir cannot be written. `action: "manual"` gives the
  command (Homebrew, cargo) or link to use instead.
- MCP: call `check_update`; tell the user to run `prybar self-update`
  and restart the MCP server.

