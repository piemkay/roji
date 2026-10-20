//! The store answers a ping from the real Postgres database named by `DATABASE_URL`;
//! `scripts/check` provides one.

use roji_store::Store;

fn database_url() -> String {
    std::env::var("DATABASE_URL").expect(
        "DATABASE_URL must name a Postgres database: run scripts/check, \
         or scripts/dev db and then eval \"$(scripts/dev env test)\"",
    )
}

#[tokio::test]
async fn pings_the_real_database() {
    let store = Store::connect(&database_url())
        .await
        .expect("connect to DATABASE_URL");
    store.ping().await.expect("ping the database");
}

#[tokio::test]
async fn ping_fails_when_the_database_is_unreachable() {
    // Nothing listens on port 1, so every connection attempt is refused.
    let store = Store::connect_lazy("postgres://roji@127.0.0.1:1/roji").expect("valid URL");
    assert!(store.ping().await.is_err());
}
