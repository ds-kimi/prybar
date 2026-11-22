# Analytics collector

`services/telemetry-collector` receives the GUI's two events (`app_start`, `crash`) and keeps only:

- `daily`: one counter per UTC day, event, version, OS and CPU type. "Active users" is the number of `app_start` per day: an estimate of launches, not of people, because no ID exists.
- `crashes`: the sanitized message and prybar frames, with day, version, OS and CPU type.

It never stores IP addresses, headers or timestamps finer than a day. The rate limit (30 requests per address per minute) keeps counts in memory only and wipes them every minute. Bodies over 16 KB, unknown fields and anything failing the schema are refused.

## Run with Docker

```sh
docker build -f services/telemetry-collector/Dockerfile -t prybar-collector .
docker run -d --name prybar-collector -p 8080:8080 \
  -v prybar-telemetry:/data \
  -e COLLECTOR_TOKEN="$(openssl rand -hex 24)" \
  prybar-collector
```

Put it behind an HTTPS reverse proxy and turn off the proxy's access log for this host, or it would keep the IPs the collector refuses to store.

| Variable | Default | Meaning |
|---|---|---|
| `COLLECTOR_ADDR` | `0.0.0.0:8080` | Listen address |
| `COLLECTOR_DB` | `telemetry.db` (`/data/telemetry.db` in Docker) | SQLite file |
| `COLLECTOR_TOKEN` | unset | Token for `/stats`, at least 16 chars; unset turns the page off |

Keep the token in the server's environment or a secret store, never in the repo.

## Stats page

```sh
curl -H "Authorization: Bearer $COLLECTOR_TOKEN" https://collector.example/stats > stats.html
```

## Point the GUI at it

The endpoint is a build-time setting. Release builds set it from a GitHub variable:

```sh
PRYBAR_TELEMETRY_URL=https://collector.example/v1/event cargo build --release -p prybar-gui
```

Without it the build is "not configured": the notice and Settings › Privacy still work and say so, and nothing is sent.

## What the app does

- Events are written to `<config dir>/telemetry/outbox/` and sent in the background at launch, oldest first; a failure keeps them for next time (at most 20 wait).
- Every event queued or sent is appended to `<config dir>/telemetry/sent.jsonl` with time, endpoint, status and the full payload. Settings › Privacy shows it, opens the file and clears it.
- Nothing is queued before the first-launch notice is answered. "Turn off" (notice or Settings) deletes the outbox at once.
- `DO_NOT_TRACK=1`, `PRYBAR_TELEMETRY=0` or `CI` set: nothing is sent, whatever the setting.
- The CLI and MCP server send no analytics.
