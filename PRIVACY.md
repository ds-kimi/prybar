# Privacy

Prybar is a tool you run on your own machine. It does not need an account and does not phone home by default, with two exceptions described here: update checks, and anonymous analytics in the desktop app (`prybar-gui`) only.

## Anonymous analytics (desktop app only)

On by default in release builds of `prybar-gui`. The first launch shows a notice with two equal buttons, "Keep on" and "Turn off". Nothing is queued or sent before you answer.

The CLI (`prybar`) and the MCP server (`prybar-mcp`) send no analytics at all.

### Exactly what is sent

Two events exist. Nothing else is sent.

`app_start`, once per launch:

```json
{"event":"app_start","version":"0.3.0","os":"macos","arch":"aarch64"}
```

`crash`, sent on the next launch after a panic:

```json
{"event":"crash","version":"0.3.0","os":"macos","arch":"aarch64","message":"...","frames":["prybar_mdl::..."]}
```

- `version`, `os`, `arch`: the app version, operating system and CPU type.
- `message`: the panic message with paths, file names, quoted text and your user name removed, at most 500 characters.
- `frames`: up to 32 backtrace frames, only from prybar's own code.

The collector refuses any other field, any value that is not short plain text, and any frame that is not prybar code.

### What is never collected

- No install ID, user ID, device ID or hardware ID. Events cannot be linked to each other or to a person.
- No file names, folder paths, model names, package contents or game names.
- No IP address is stored. The collector keeps no headers and no timestamps finer than a day. The rate limiter holds addresses in memory only and wipes them every minute.
- No usage tracking of screens, buttons or features.

"Active users" is the number of `app_start` events per day, version and OS. It is an estimate of launches, not of people.

### Where it goes

To the collector the maintainers run, set at build time (`PRYBAR_TELEMETRY_URL`). A build without it is "not configured" and sends nothing. The collector source is in [services/telemetry-collector](services/telemetry-collector) and its deployment notes are in [docs/privacy/collector.md](docs/privacy/collector.md).

### Retention

The collector stores one counter per day, event, version, OS and CPU type, and the sanitized crash reports. It does not delete them automatically today; they are kept until the operator removes them. Only the last 60 days of counts appear on its stats page.

### Turn it off

Any one of these is enough, and it takes effect at once:

- In the app: the first-launch notice, or Settings > Privacy. Turning it off deletes queued reports.
- Environment: `PRYBAR_TELEMETRY=0` or `DO_NOT_TRACK=1`.
- `CI` set: always off.

### See what was sent

Every event queued or sent is appended to `sent.jsonl` in the `telemetry` folder next to the app's config (set `PRYBAR_CONFIG` to move it). Settings > Privacy lists each entry with its time and size; click one to see the full JSON. The same page opens the file and clears it.

## Update checks

The desktop app checks for a new release at launch; the CLI and MCP server check at most once a day. This is a plain request to the GitHub releases API and sends nothing about you or your files. Downloads are verified with SHA256 and a minisign signature before anything is replaced.

Turn it off with `PRYBAR_NO_UPDATE_CHECK=1` (also off in CI), or the auto-update switch in the app's Settings > Updates.

## Questions

Open an issue at https://github.com/ds-kimi/prybar/issues.
