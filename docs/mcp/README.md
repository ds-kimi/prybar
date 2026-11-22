# prybar-mcp

MCP server over stdio (rmcp). Same operations as the CLI, through the
shared `prybar-service` crate.

## Client config

```json
{ "mcpServers": { "prybar": { "command": "/abs/path/to/prybar-mcp" } } }
```

## Tools

| Tool | Does |
|---|---|
| `inspect_model` | header summary of one model, no files written |
| `decompile_model` | 1-64 inputs (files, folders, packages, `pkg:inner`, wildcards) to QC/SMD/VTA/VRD; counts + first 50 results/failures |
| `list_package` | sorted entries; `filter`, `limit` (default 200, max 5000); `total` counts all |
| `unpack_package` | extract a whole package; returns the file count |
| `pack_package` | folder to `.vpk` (v1/v2, chunks) or `.gma` (needs `addon.json`) |
| `compile_model` | studiomdl on a QC; a failed compile is a result with `success: false` |
| `view_model` | start hlmv, returns its pid |
| `publish_workshop` | spec file publish/update (`dry_run`), or `list_app_id` |
| `games_list` | configured games, `detect` Steam installs, `save` |
| `config_get` | config path + games |
| `config_set` | add/replace/remove one game (name, game_dir, packer) |
| `check_update` | `{current, latest, update_available, command}`; never installs (the user runs `prybar self-update`) |

Agent-facing reference (errors, recipes, troubleshooting): `docs/agents/`. Client snippets: `docs/agents/mcp-config.md`.

## Rules

- Updates: at start the server refreshes a daily cached release check in
  the background; when a newer release is cached, the `initialize`
  instructions say so. The server never replaces itself. Off with
  `PRYBAR_NO_UPDATE_CHECK=1` or `CI`.

- Every path is absolute. Windows UNC and device paths (`\host\share`,
  `\?\UNC\...`, `\.\pipe\...`) are refused: opening one leaks NTLM
  credentials.
- Workshop spec `preview` and `content` paths must be absolute too.
- `config_set` cannot set `studiomdl` or `hlmv` (unknown fields are
  refused). `compile_model`/`view_model` run those paths and
  `unpack_package` can write files anywhere, so a prompt-injected agent
  could otherwise run a program it wrote. Tool paths come from
  `games_list` with `detect` + `save`, the CLI, or a hand edit of the
  config file; `config_set` keeps the ones already there.
- Errors are tool results with `isError: true` and
  `{ "error", "kind", "code" }`; `code` is the CLI exit code
  (`docs/cli/README.md`). Arguments that break the JSON schema are
  rejected by rmcp before prybar runs.

## Manual handshake

```sh
cargo build -p prybar-mcp
./target/debug/prybar-mcp < scripts/mcp/handshake.jsonl
```

Prints the `initialize` answer and the `tools/list` answer, then exits
when stdin closes.
