//! Postgres storage for Roji. This is the only crate that talks to the database; the server
//! reaches it through [`Store`].

#![forbid(unsafe_code)]

use std::fmt;
use std::time::Duration;

use sqlx::postgres::{PgPool, PgPoolOptions};

/// How long a caller waits for a database connection before giving up.
const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(3);

/// A pool of connections to the Roji database.
#[derive(Debug, Clone)]
pub struct Store {
    pool: PgPool,
}

impl Store {
    /// Connects to the database at `url`, failing when it cannot be reached.
    pub async fn connect(url: &str) -> Result<Self, StoreError> {
        let pool = pool_options().connect(url).await?;
        Ok(Self { pool })
    }

    /// Prepares a pool for the database at `url` without connecting yet; connections open on
    /// first use, so a server can start while the database is still down.
    pub fn connect_lazy(url: &str) -> Result<Self, StoreError> {
        let pool = pool_options().connect_lazy(url)?;
        Ok(Self { pool })
    }

    /// Checks that the database answers a query.
    pub async fn ping(&self) -> Result<(), StoreError> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}

fn pool_options() -> PgPoolOptions {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(ACQUIRE_TIMEOUT)
}

/// A failed database operation.
#[derive(Debug)]
pub struct StoreError(sqlx::Error);

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "database error: {}", self.0)
    }
}

impl std::error::Error for StoreError {}

impl From<sqlx::Error> for StoreError {
    fn from(err: sqlx::Error) -> Self {
        Self(err)
    }
}
