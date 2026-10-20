//! Starts the Roji server.
//!
//! Configuration comes from the environment:
//! - `DATABASE_URL` (required): the Postgres database.
//! - `ROJI_ADDR` (default `127.0.0.1:8080`): where to listen; port 0 picks a free port.
//!
//! Once listening, the server prints `listening on <address>` as its first line on stdout.
//! Logs go to stderr.

#![forbid(unsafe_code)]

use std::env;

use anyhow::Context;
use roji_store::Store;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let database_url = env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
    let addr = env::var("ROJI_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_owned());

    let store = Store::connect_lazy(&database_url).context("DATABASE_URL is not usable")?;
    let listener = TcpListener::bind(&addr)
        .await
        .with_context(|| format!("cannot listen on {addr}"))?;
    println!("listening on {}", listener.local_addr()?);

    axum::serve(listener, roji_server::router(store))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server failed")
}

/// Resolves on Ctrl-C or SIGTERM, so the server finishes open requests before it exits.
async fn shutdown_signal() {
    let interrupt = async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            tracing::error!(%err, "cannot listen for Ctrl-C");
            std::future::pending::<()>().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(err) => {
                tracing::error!(%err, "cannot listen for SIGTERM");
                std::future::pending::<()>().await;
            }
        }
    };
    tokio::select! {
        () = interrupt => {},
        () = terminate => {},
    }
}
