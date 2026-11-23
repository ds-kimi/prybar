# AGENTS.md

Prybar decompiles and packages Source engine content (MDL 04-53, VPK, GMA, APK, HFS) for people and AI agents. Codex, ChatGPT, Cursor and other agents can use it through the CLI `prybar` or the MCP server `prybar-mcp` (`docs/agents/mcp-config.md`).

## Using prybar

```sh
prybar --json inspect <model.mdl>
prybar --json decompile <inputs...> -o <dir> [-r] [--split-qc]
prybar --json list <package> [--filter "models/*.mdl"] [--limit N]
prybar --json unpack <package> -o <dir>
prybar --json pack <dir> -o <out.vpk|out.gma>
prybar --json games [--detect --save]
prybar --json compile <file.qc> -g <game>
prybar publish <spec.json> --dry-run
prybar --json workshop items <game> [--search T] [--sort subscribed]
prybar --json workshop item <game> <id>
prybar --json workshop bbcode <desc.txt>
prybar --json texture <file.vtf> -o <out.png>
prybar --json privacy
```

- `--json` gives one JSON document on stdout; progress goes to stderr.
- Exit codes: 0 ok, 1 internal, 2 usage, 3 not found, 4 invalid input, 5 I/O, 6 external tool, 7 partial batch.
- Model inputs: a `.mdl`, a folder, a wildcard, or `pak_dir.vpk:models/props/crate.mdl`.
- Keep companions (`.vvd`, `.vtx`, `.phy`, `.ani`) next to the `.mdl`.
- Do not commit or redistribute decompiled game assets.

Docs for agents: `docs/agents/usecases.md` (recipes), `docs/agents/parity.md` (GUI/CLI/MCP feature table), `docs/agents/errors.md` (every error and fix), `docs/agents/troubleshooting.md`. Claude Code skill: `plugins/prybar/skills/prybar/SKILL.md`.

## Working on the repo

Read `CONTRIBUTING.md` first. Checks before every commit:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
sh scripts/check-limits/check-limits.sh
```

Rules: at most 4 functions per `.rs` file, at most 30 lines per function, at most 10 `.rs` files per folder (nested module folders by domain). English only, no trailing spaces, comments say why. Untrusted input: bounds-check everything, never panic. Conventional commits.
