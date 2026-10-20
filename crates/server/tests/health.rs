//! `GET /api/health` reports whether the server reaches its database.
//!
//! The first test starts the real `roji-server` binary against the real Postgres database named
//! by `DATABASE_URL`; `scripts/check` provides one.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use roji_store::Store;
use serde_json::{Value, json};
use tower::ServiceExt;

fn database_url() -> String {
    std::env::var("DATABASE_URL").expect(
        "DATABASE_URL must name a Postgres database: run scripts/check, \
         or scripts/dev up and then eval \"$(scripts/dev env)\"",
    )
}

/// Stops the server when the test ends, also when it fails.
struct Running(Child);

impl Drop for Running {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
fn the_server_binary_is_healthy_with_the_real_database() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_roji-server"))
        .env("DATABASE_URL", database_url())
        .env("ROJI_ADDR", "127.0.0.1:0")
        .stdout(Stdio::piped())
        .spawn()
        .expect("start roji-server");
    let stdout = child.stdout.take().expect("piped stdout");
    let _running = Running(child);

    let mut line = String::new();
    BufReader::new(stdout)
        .read_line(&mut line)
        .expect("read the first line of the server");
    let addr = line
        .trim()
        .strip_prefix("listening on ")
        .unwrap_or_else(|| panic!("the server did not start; it printed {line:?}"));

    let (status, body) = http_get(addr, "/api/health");
    assert_eq!(status, 200, "body: {body}");
    let health: Value = serde_json::from_str(&body).expect("JSON body");
    assert_eq!(health, json!({ "status": "ok", "database": "ok" }));
}

#[tokio::test]
async fn health_is_unavailable_when_the_database_is_down() {
    // Nothing listens on port 1, so every connection attempt is refused.
    let store = Store::connect_lazy("postgres://roji@127.0.0.1:1/roji").expect("valid URL");
    let response = roji_server::router(store)
        .oneshot(
            Request::get("/api/health")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let health: Value = serde_json::from_slice(&body).expect("JSON body");
    assert_eq!(
        health,
        json!({ "status": "unavailable", "database": "unreachable" })
    );
}

/// A minimal HTTP/1.1 GET, so the test needs no HTTP client library.
fn http_get(addr: &str, path: &str) -> (u16, String) {
    let mut stream = TcpStream::connect(addr).expect("connect to the server");
    write!(
        stream,
        "GET {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n"
    )
    .expect("send request");
    let mut response = String::new();
    stream.read_to_string(&mut response).expect("read response");
    let (head, body) = response.split_once("\r\n\r\n").expect("an HTTP response");
    let status = head
        .split_whitespace()
        .nth(1)
        .and_then(|code| code.parse().ok())
        .expect("a status code");
    (status, body.to_owned())
}
