# Using prybar from ChatGPT, Codex and other agents

Two ways in. Pick the one your agent supports.

## Codex CLI / ChatGPT agents with MCP

Register `prybar-mcp` in `~/.codex/config.toml`:

```toml
[mcp_servers.prybar]
command = "/abs/path/to/prybar-mcp"
args = []
```

Other clients: `docs/agents/mcp-config.md`. Every path given to a tool must be absolute.

## Agents with a shell only (no MCP)

Call the CLI with `--json`. One JSON document goes to stdout, progress to stderr.

```sh
prybar --json inspect /abs/path/model.mdl
prybar --json decompile /abs/path/model.mdl -o /abs/out
prybar --json list /abs/path/pak01_dir.vpk --filter "models/*.mdl" --limit 20
```

Branch on the exit code: 0 ok, 1 internal, 2 usage, 3 not found, 4 invalid input,
5 I/O, 6 external tool, 7 partial batch. Each error and its fix: `docs/agents/errors.md`.

## Rules for agents

- Keep `.vvd`, `.vtx`, `.phy` and `.ani` next to the `.mdl`.
- Do not commit or redistribute decompiled game assets.
- Compile and view need a configured game: run `prybar --json games --detect --save` first.
- Recipes: `docs/agents/usecases.md`. Problems: `docs/agents/troubleshooting.md`.
