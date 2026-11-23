# Errors

Every failure carries a `code`, the same number as the CLI exit code. In the CLI with `--json`: `{"error": {"message": "...", "code": N}}`. In MCP: a tool result with `isError: true` and `{"error": "...", "kind": "...", "code": N}`. A batch (`decompile`) never stops on one bad model: it reports `failures`, each with its own code.

| code | kind | Meaning | Next step |
|---|---|---|---|
| 0 | `ok` | success | none |
| 1 | `internal` | bug or unexpected state | rerun once; report it with the command and input |
| 2 | `usage` | bad arguments, unknown game, missing tool path, refused path, read-only format | fix the argument named in the message |
| 3 | `not_found` | input file, folder or package entry missing | check the path with `list` (packages) or the file system |
| 4 | `invalid_input` | malformed model, package, spec or config | the file is damaged or not that format; see the messages below |
| 5 | `io` | cannot read the input or write the output | check permissions and free space; pick another output folder |
| 6 | `tool` | studiomdl, hlmv or Steam failed or did not start | see `troubleshooting.md` |
| 7 | `partial` | batch where some models were written and some failed | read `failures`; fix or skip them |

When nothing was decompiled the run exits with the failures' shared code (4 if they differ). Code 7 means at least one model was written.

## Messages by kind

### usage (2)

| Message | Cause and fix |
|---|---|
| ``unknown game `X` `` | no game of that name; run `games`, then `games --detect --save` |
| ``game `X` has no studiomdl path configured`` (or `hlmv`) | edit the config file by hand or run `games --detect --save`; MCP `config_set` cannot set tool paths |
| `<field>: `V` is not an absolute local path` | MCP paths must be absolute; UNC and device paths are refused |
| `<field>: expected a path of 1 to N bytes` | empty or oversized path |
| `inputs: expected 1 to 64 items` | send at most 64 inputs per `decompile_model` call |
| ``built without the `workshop` feature; rebuild with --features workshop`` | `publish` needs a build with the Steam SDK |
| `invalid <field>: <reason>` | Workshop spec field is wrong; fix the spec JSON (`content` must be an existing `.gma` or addon folder with a valid `addon.json` and only whitelisted files for GMod app 4000, else an existing folder) |
| `not a supported Workshop game; use 4000 (garrysmod) or 440 (tf2)` | the spec's `app_id` is not a game prybar can publish to; see `publish --games` |
| `<game> uses the legacy ... Workshop` / `has no Steam Workshop` | that game cannot take uploads from prybar; use the tool named in the message |
| `content: must be a folder with at least one compiled map (.bsp)...` | Team Fortress 2 items are map folders: put the `.bsp` in the folder or its `maps/` |
| `a real publish needs `confirm: true`` (MCP) | `publish_workshop` without `dry_run` needs `confirm: true`; dry run first and ask the user |
| ``unknown sort `X` `` / ``unknown visibility `X` `` | `workshop items`: use the values the message lists |
| `changing privacy settings needs `confirm: true`` (MCP) | `privacy` with `set` or `clear_log` changes the user's settings; ask first |
| `no user config directory on this system` | set `PRYBAR_CONFIG` to a file path |
| read-only format | `pack` only writes `.vpk` and `.gma`; APK and HFS are read-only |

### not_found (3)

`not found: <path>`: the model, package, QC, spec or the inner entry of `pkg:inner` does not exist. Use `list --filter` to see the real entry names. Also `not found: materials/<name>.vtf` (`texture --material`: no search root holds it; configure the game with `games --detect --save`) and `not found: Workshop item N among your items` (`workshop item`: wrong id or game, or not your item).

### invalid_input (4)

Format of the message: `<input>: <detail>`.

| Detail | Cause and fix |
|---|---|
| `not an MDL file (magic ...)` | wrong file; check with `inspect` on another one |
| `MDL version N is not supported yet` | version outside 04-53 or a variant not implemented |
| `<what> has invalid value N` | corrupt or modified model; check the `.vvd`/`.vtx` match the `.mdl` |
| `read of N bytes at offset O exceeds buffer of S bytes` | truncated file |
| `count N exceeds sanity cap C` | corrupt count field; the file is damaged or hostile |
| `string at offset O has no terminator within M bytes` | corrupt string table |
| `string at offset O is not valid UTF-8` | corrupt string table |
| `<file.vtf>: ...` | `texture`: not a VTF, truncated, or a format not decoded; the message names it |
| `<image>: ...` | `workshop preview`: not a readable PNG, JPEG or GIF |
| `not a VPK directory file` | pass the `_dir.vpk`, not a `_000.vpk` chunk |
| `unsupported VPK version N` | only v1 and v2 |
| `bad entry terminator at offset O` | damaged VPK directory |
| `multi-archive output name must end with _dir.vpk` | `--chunk-size` needs `-o name_dir.vpk` |
| `too many archives or data too large for VPK` | raise `--chunk-size` |
| `CRC mismatch for <file>` | damaged package or entry (VPK, GMA) |
| `not a GMA file`, `unsupported GMA version N` | wrong file or newer format |
| `entry "<name>" has an invalid size` | damaged GMA |
| `files not allowed by the gmod whitelist: [...]` | remove those files from the folder before packing a `.gma` |
| `addon.json: <detail>` | GMA packing needs a valid `addon.json` in the source folder |
| `unsafe entry path "<p>"` | entry escapes the output folder (`..`, absolute path); the package is hostile, that entry is not written |
| `path is not valid UTF-8: ...` | rename the file; names must be UTF-8 |
| `not an APK package` | wrong file |
| `not an HFS package: no end-of-directory record` | wrong or truncated file |
| `bad record signature 0x... at offset O` | damaged HFS |
| `"<name>" is a .comp entry; its compression is not supported` | HFS `.comp` entries are listed, not extracted |
| `invalid config <path>: <detail>` | config TOML is broken; fix or delete it |
| `larger than 1 MiB` | the spec or config file is not a real one |

### io (5)

`io error on <path>: <os error>`: permission denied, disk full, file in use, or an output path that is a file where a folder is needed. Directory-walk failures end here too.

### tool (6)

| Message | Cause and fix |
|---|---|
| `cannot start <exe>: <os error>` | the configured studiomdl or hlmv path does not exist or is not executable (on Linux and macOS it is a Windows exe: use Wine) |
| `steam: <detail>` | Steam is not running, not signed in, or the app is not owned; start Steam and retry |
| `tool task failed: ...` | the task crashed; retry |
| `update check: <detail>` | `self-update` could not reach GitHub; check the network and retry |
| `network: ...`, `no release asset for ...`, `cannot install here: ...` | `self-update` download or install failed; use the release page |

A compile that ran but failed is not an MCP error: `compile_model` returns `success: false` with the studiomdl log. The CLI exits 6 for it.

### partial (7) and internal (1)

`partial` has no message of its own: read `failures` in the report. `internal` is a bug: keep the command and the input.
