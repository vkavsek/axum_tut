mod error;

use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub use self::error::{Error, Result};

use crate::config;

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    // See NOTE below.
    let max_connections = if cfg!(test) { 1 } else { 5 };

    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_millis(500))
        .connect(&config().DB_URL)
        .await
        .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}

// NOTE: 1) This is not an ideal situation; however, with sqlx 0.7, when executing `cargo test`, some tests that use sqlx fail at a
//          rather low level (in the tokio scheduler). It appears to be a low-level thread/async issue, as removing/adding
//          tests causes different tests to fail. The cause remains uncertain, but setting max_connections to 1 resolves the issue.
//          The good news is that max_connections still function normally for a regular run.
//          This issue is likely due to the unique requirements unit tests impose on their execution, and therefore,
//          while not ideal, it should serve as an acceptable temporary solution.
//
// NOTE: 2) Apparently an alternative solution could be to wrap all the test functions that use the same runtime in a single function
//          and initialize the runtime there.
//          The problem seems to be that every `tokio::test` runs in a separate runtime, but share the same OnceCell initialized by one of them.
//          If the runtime which initialized the OnceCell exits and the IO resources just stop working the other runtimes can never get the OnceCell.
//
