//! Collector for prybar GUI analytics. Configured by environment only:
//! `COLLECTOR_ADDR` (default `0.0.0.0:8080`), `COLLECTOR_DB` (default
//! `telemetry.db`), `COLLECTOR_TOKEN` (stats page; unset = page off).

mod app;
mod day;
mod db;
mod ingest;
mod stats;
#[cfg(test)]
mod tests;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    if let Err(e) = serve().await {
        eprintln!("telemetry-collector: {e}");
        std::process::exit(1);
    }
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let var = |k: &str, d: &str| std::env::var(k).unwrap_or_else(|_| d.into());
    let addr = var("COLLECTOR_ADDR", "0.0.0.0:8080");
    let conn = db::open(&var("COLLECTOR_DB", "telemetry.db"))?;
    let token = std::env::var("COLLECTOR_TOKEN")
        .ok()
        .filter(|t| t.len() >= 16);
    if token.is_none() {
        eprintln!("telemetry-collector: COLLECTOR_TOKEN unset or under 16 chars, stats page off");
    }
    let router = app::router(app::State::new(conn, token));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    eprintln!("telemetry-collector: listening on {addr}");
    let service = router.into_make_service_with_connect_info::<SocketAddr>();
    axum::serve(listener, service).await?;
    Ok(())
}
