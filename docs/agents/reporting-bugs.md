# Reporting a prybar bug as an agent

A prybar bug is wrong output, a crash, exit code 1 (internal), or an MCP
tool error that is not caused by the user's input. Bad paths, missing
companions and unsupported versions are input problems: see
`errors.md` and `troubleshooting.md` first.

## Flow (two explicit yeses)

1. Stop and ask: "Want me to open an issue on ds-kimi/prybar?" Only an
   explicit yes continues. Silence or a vague answer means no.
2. Check `gh` is installed and authenticated: `gh auth status`. If not,
   give the user the prefilled title and body below plus
   <https://github.com/ds-kimi/prybar/issues/new?template=1_bug.yml>
   and stop.
3. Search for duplicates: `gh issue list -R ds-kimi/prybar --state all --search "<key words>"`.
   If one matches, offer to comment on it instead.
4. Draft the issue with the bug form fields:
   - prybar version (`prybar --version`), OS, surface (GUI, CLI, MCP), game
   - exact command or MCP tool call
   - expected vs actual
   - `--json` output or the MCP error JSON
   - MDL version from `prybar --json inspect` for model bugs (use the
     model form `2_model.yml` then)
   Redact private paths and usernames (`/Users/<name>/` -> `~/`). Never
   attach game files.
5. Show the exact title, body and labels (`bug`, `needs triage`, plus one
   `area: ...`), and ask a second time. Only an explicit yes continues.
6. Run `gh issue create -R ds-kimi/prybar --title "..." --body-file <file> --label bug --label "needs triage"`
   and give the user the link.

Never file without both yeses. Never file more than one issue per bug.

## Labels

Labels live in `.github/labels.yml` and are synced with
`sh scripts/github/sync-labels.sh`. Groups: type (`bug`, `regression`,
`enhancement`, `docs`, `question`), area (`area: ...`), status
(`needs triage`, `needs repro`, `needs sample`, `confirmed`, `blocked`),
priority (`P1`..`P3`), and `good first issue`, `help wanted`,
`duplicate`, `wontfix`.
