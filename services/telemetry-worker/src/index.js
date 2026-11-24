// Prybar analytics collector on Cloudflare Workers + D1.
// Same contract as services/telemetry-collector: two event kinds, strict
// schema, day-level time only, no address or header is ever read or stored.
// Public: GET /stats (aggregate counts). Private: GET /crashes (token).

const MAX_BODY = 16 * 1024;
const MAX_MESSAGE = 500;
const MAX_FRAMES = 32;

const short = (s) => typeof s === "string" && s.length > 0 && s.length <= 32 && /^[\x21-\x7e]+$/.test(s);

function valid(e) {
  if (!e || typeof e !== "object" || Array.isArray(e)) return false;
  const keys = Object.keys(e).sort().join(",");
  if (e.event === "app_start") return keys === "arch,event,os,version" && short(e.version) && short(e.os) && short(e.arch);
  if (e.event === "crash")
    return keys === "arch,event,frames,message,os,version" && short(e.version) && short(e.os) && short(e.arch)
      && typeof e.message === "string" && new TextEncoder().encode(e.message).length <= MAX_MESSAGE
      && Array.isArray(e.frames) && e.frames.length <= MAX_FRAMES
      && e.frames.every((f) => typeof f === "string" && f.length <= 200 && f.startsWith("prybar"));
  return false;
}

async function ingest(req, env) {
  const buf = await req.arrayBuffer();
  if (buf.byteLength > MAX_BODY) return new Response(null, { status: 413 });
  let e;
  try { e = JSON.parse(new TextDecoder().decode(buf)); } catch { return new Response(null, { status: 400 }); }
  if (!valid(e)) return new Response(null, { status: 400 });
  const day = new Date().toISOString().slice(0, 10);
  const stmts = [env.DB.prepare(
    "INSERT INTO daily VALUES (?1, ?2, ?3, ?4, ?5, 1) ON CONFLICT DO UPDATE SET count = count + 1",
  ).bind(day, e.event, e.version, e.os, e.arch)];
  if (e.event === "crash")
    stmts.push(env.DB.prepare(
      "INSERT INTO crashes (day, version, os, arch, message, frames) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    ).bind(day, e.version, e.os, e.arch, e.message, JSON.stringify(e.frames)));
  await env.DB.batch(stmts);
  return new Response(null, { status: 204 });
}

const esc = (s) => String(s).replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
const table = (head, rows) =>
  `<table><tr>${head.map((h) => `<th>${h}</th>`).join("")}</tr>${rows
    .map((r) => `<tr>${r.map((c) => `<td>${esc(c)}</td>`).join("")}</tr>`).join("")}</table>`;
const page = (title, body) => `<!doctype html><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>${title}</title><style>
:root{color-scheme:light dark;--bg:#fff;--fg:#1b1b1b;--mut:#6b6b6b;--line:#e4e4e4;--acc:#b7410e}
@media (prefers-color-scheme:dark){:root{--bg:#141414;--fg:#ececec;--mut:#9a9a9a;--line:#2a2a2a;--acc:#e0743f}}
body{background:var(--bg);color:var(--fg);font:15px/1.5 system-ui,sans-serif;max-width:860px;margin:40px auto;padding:0 16px}
h1{font-size:22px;margin:0 0 4px}h2{font-size:16px;margin:32px 0 8px}p{color:var(--mut);margin:0 0 16px}
a{color:var(--acc)}table{border-collapse:collapse;width:100%;font-variant-numeric:tabular-nums}
th,td{text-align:left;padding:6px 8px;border-bottom:1px solid var(--line)}th{color:var(--mut);font-weight:500}
.big{display:flex;gap:32px;margin:16px 0}.big b{display:block;font-size:28px}.big span{color:var(--mut);font-size:13px}
.wrap{overflow-x:auto}</style>${body}`;

async function stats(env) {
  const q = (sql) => env.DB.prepare(sql).all().then((r) => r.results);
  const [days, versions, platforms, totals] = await Promise.all([
    q(`SELECT day, SUM(CASE WHEN event='app_start' THEN count END) AS launches,
       SUM(CASE WHEN event='crash' THEN count END) AS crashes
       FROM daily WHERE day >= date('now','-60 days') GROUP BY day ORDER BY day DESC`),
    q(`SELECT version, SUM(CASE WHEN event='app_start' THEN count END) AS launches,
       SUM(CASE WHEN event='crash' THEN count END) AS crashes
       FROM daily WHERE day >= date('now','-30 days') GROUP BY version ORDER BY launches DESC`),
    q(`SELECT os, arch, SUM(count) AS launches FROM daily
       WHERE event='app_start' AND day >= date('now','-30 days') GROUP BY os, arch ORDER BY launches DESC`),
    q(`SELECT SUM(CASE WHEN event='app_start' AND day >= date('now','-30 days') THEN count END) AS m,
       SUM(CASE WHEN event='app_start' AND day = date('now') THEN count END) AS d FROM daily`),
  ]);
  const n = (v) => v ?? 0;
  const t = totals[0] || {};
  return new Response(page("Prybar stats", `<h1>Prybar stats</h1>
<p>Public, anonymous counts from the desktop app: launches and crashes per day, version and platform. No IDs, no IP addresses, nothing finer than a day. A launch is not a person. <a href="https://github.com/ds-kimi/prybar/blob/main/PRIVACY.md">Privacy</a> · <a href="https://github.com/ds-kimi/prybar">GitHub</a></p>
<div class="big"><div><b>${n(t.d)}</b><span>launches today (UTC)</span></div><div><b>${n(t.m)}</b><span>launches, last 30 days</span></div></div>
<h2>Per day (60 days)</h2><div class="wrap">${table(["Day", "Launches", "Crashes"], days.map((r) => [r.day, n(r.launches), n(r.crashes)]))}</div>
<h2>Per version (30 days)</h2><div class="wrap">${table(["Version", "Launches", "Crashes"], versions.map((r) => [r.version, n(r.launches), n(r.crashes)]))}</div>
<h2>Per platform (30 days)</h2><div class="wrap">${table(["OS", "CPU", "Launches"], platforms.map((r) => [r.os, r.arch, n(r.launches)]))}</div>`),
    { headers: { "content-type": "text/html; charset=utf-8", "cache-control": "public, max-age=300" } });
}

function same(a, b) {
  const x = new TextEncoder().encode(a), y = new TextEncoder().encode(b);
  let d = x.length ^ y.length;
  for (let i = 0; i < Math.max(x.length, y.length); i++) d |= (x[i] ?? 0) ^ (y[i] ?? 0);
  return d === 0;
}

async function crashes(req, env) {
  const tok = env.COLLECTOR_TOKEN;
  const got = (req.headers.get("authorization") || "").replace(/^Bearer /, "");
  if (!tok || tok.length < 16 || !same(got, tok)) return new Response("not found", { status: 404 });
  const rows = (await env.DB.prepare(
    "SELECT day, version, os, arch, message, frames FROM crashes ORDER BY id DESC LIMIT 100").all()).results;
  return new Response(page("Prybar crashes", `<h1>Crashes (private)</h1><div class="wrap">${table(
    ["Day", "Version", "OS", "CPU", "Message", "Frames"],
    rows.map((r) => [r.day, r.version, r.os, r.arch, r.message, JSON.parse(r.frames).join("\n")]))}</div>`),
    { headers: { "content-type": "text/html; charset=utf-8", "cache-control": "no-store" } });
}

export default {
  async fetch(req, env) {
    const { pathname } = new URL(req.url);
    if (req.method === "POST" && pathname === "/v1/event") return ingest(req, env);
    if (req.method === "GET" && (pathname === "/" || pathname === "/stats")) return stats(env);
    if (req.method === "GET" && pathname === "/crashes") return crashes(req, env);
    return new Response("not found", { status: 404 });
  },
};
