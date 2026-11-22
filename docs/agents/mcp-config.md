# MCP client configuration

Build or download `prybar-mcp`, then register the absolute path of the binary. It speaks MCP over stdio and needs no arguments. Set `PRYBAR_CONFIG` in the client's `env` to use a config file other than the default.

## Claude Code

```sh
claude mcp add prybar -- /abs/path/to/prybar-mcp
```

## Claude Desktop (`claude_desktop_config.json`)

```json
{
  "mcpServers": {
    "prybar": { "command": "C:\\tools\\prybar\\prybar-mcp.exe" }
  }
}
```

## Cursor (`.cursor/mcp.json` or `~/.cursor/mcp.json`)

```json
{
  "mcpServers": {
    "prybar": { "command": "/abs/path/to/prybar-mcp", "args": [] }
  }
}
```

## VS Code (`.vscode/mcp.json`)

```json
{
  "servers": {
    "prybar": { "type": "stdio", "command": "/abs/path/to/prybar-mcp" }
  }
}
```

## Codex / ChatGPT agents (`~/.codex/config.toml`)

```toml
[mcp_servers.prybar]
command = "/abs/path/to/prybar-mcp"
args = []
```

Agents without MCP support use the CLI instead: see `AGENTS.md` at the repo root.

## Check it works

```sh
./prybar-mcp < scripts/mcp/handshake.jsonl
```

It prints the `initialize` and `tools/list` answers, then exits. Every path passed to a tool must be absolute.
