use std::time::Duration;

use diesel::ConnectionError;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use tokio_postgres_rustls::MakeRustlsConnect;

pub type DbPool = Pool<AsyncPgConnection>;

/// `AsyncPgConnection::establish` hard-codes `tokio_postgres::NoTls`, which
/// hangs (rather than failing fast) against providers like Neon that
/// require TLS. This wires up a real rustls connector instead, per
/// diesel-async's documented pattern for TLS-only Postgres hosts.
fn establish(database_url: &str) -> BoxFuture<'_, Result<AsyncPgConnection, ConnectionError>> {
    let mut roots = rustls::RootCertStore::empty();
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let tls_config = rustls::ClientConfig::builder().with_root_certificates(roots).with_no_client_auth();
    let tls = MakeRustlsConnect::new(tls_config);

    async move {
        let (client, conn) = tokio_postgres::connect(database_url, tls)
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
        AsyncPgConnection::try_from_client_and_connection(client, conn).await
    }
    .boxed()
}

/// Serverless Postgres (Neon) suspends its compute after a few minutes idle
/// and can take longer than bb8's 30s default to wake back up, so give new
/// connections more room before giving up.
pub async fn build_pool(database_url: &str) -> anyhow::Result<DbPool> {
    let mut manager_config = ManagerConfig::default();
    manager_config.custom_setup = Box::new(|url| establish(url));

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(database_url, manager_config);
    let pool = Pool::builder().max_size(10).connection_timeout(Duration::from_secs(60)).build(manager).await?;
    Ok(pool)
}

/// Runs a trivial query on an interval to keep the Neon compute from
/// suspending during normal operation, so most requests don't pay the
/// cold-start cost. Best-effort: a failed ping just means the next real
/// request eats the wake-up latency instead.
pub fn spawn_keepalive(pool: DbPool) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(4 * 60));
        loop {
            interval.tick().await;
            if let Ok(mut conn) = pool.get().await {
                let _ = diesel::sql_query("SELECT 1").execute(&mut *conn).await;
            }
        }
    });
}
