//! SQLite storage: one counter row per day/event/version/os/arch, and
//! crash reports. No column can hold an address or a header.

use prybar_telemetry::Event;
use rusqlite::{params, Connection};

/// Table rows as display cells.
pub type Rows = Vec<Vec<String>>;

pub fn open(path: &str) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS daily (
            day TEXT NOT NULL, event TEXT NOT NULL, version TEXT NOT NULL,
            os TEXT NOT NULL, arch TEXT NOT NULL, count INTEGER NOT NULL,
            PRIMARY KEY (day, event, version, os, arch));
         CREATE TABLE IF NOT EXISTS crashes (
            id INTEGER PRIMARY KEY, day TEXT NOT NULL, version TEXT NOT NULL,
            os TEXT NOT NULL, arch TEXT NOT NULL, message TEXT NOT NULL,
            frames TEXT NOT NULL);",
    )?;
    Ok(conn)
}

/// Counts every event; a crash also keeps its sanitized report.
pub fn record(conn: &Connection, day: &str, event: &Event) -> rusqlite::Result<()> {
    let (name, version, os, arch) = match event {
        Event::AppStart { version, os, arch } => ("app_start", version, os, arch),
        Event::Crash {
            version, os, arch, ..
        } => ("crash", version, os, arch),
    };
    conn.execute(
        "INSERT INTO daily VALUES (?1, ?2, ?3, ?4, ?5, 1)
         ON CONFLICT DO UPDATE SET count = count + 1",
        params![day, name, version, os, arch],
    )?;
    if let Event::Crash {
        message, frames, ..
    } = event
    {
        let frames = serde_json::to_string(frames).unwrap_or_default();
        conn.execute(
            "INSERT INTO crashes (day, version, os, arch, message, frames)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![day, version, os, arch, message, frames],
        )?;
    }
    Ok(())
}

/// Rows for the stats page: the last 60 days of counts, newest first,
/// then the last 50 crashes. Each row is already a list of cells.
pub fn read(conn: &Connection) -> rusqlite::Result<(Rows, Rows)> {
    let rows = |sql: &str, cols: usize| -> rusqlite::Result<Rows> {
        let mut stmt = conn.prepare(sql)?;
        let out = stmt.query_map([], |r| {
            (0..cols)
                .map(|i| r.get::<_, rusqlite::types::Value>(i).map(cell))
                .collect()
        })?;
        out.collect()
    };
    let daily = rows(
        "SELECT day, event, version, os, arch, count FROM daily
         WHERE day >= date('now', '-60 days') ORDER BY day DESC, event, version",
        6,
    )?;
    let crashes = rows(
        "SELECT day, version, os, arch, message, frames FROM crashes ORDER BY id DESC LIMIT 50",
        6,
    )?;
    Ok((daily, crashes))
}

fn cell(v: rusqlite::types::Value) -> String {
    match v {
        rusqlite::types::Value::Integer(i) => i.to_string(),
        rusqlite::types::Value::Text(s) => s,
        _ => String::new(),
    }
}
