//! Migrations run through a plain sync `PgConnection` at startup, kept
//! separate from the async `bb8` pool used for request handling. Diesel's
//! migration harness doesn't have an async variant.

use diesel::pg::PgConnection;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations");

pub fn run_pending_migrations(database_url: &str) -> anyhow::Result<()> {
    let mut conn = PgConnection::establish(database_url)?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("failed to run migrations: {e}"))?;
    Ok(())
}
