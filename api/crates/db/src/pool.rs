use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type DbPool = Pool<AsyncPgConnection>;

pub async fn build_pool(database_url: &str) -> anyhow::Result<DbPool> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = Pool::builder().max_size(10).build(manager).await?;
    Ok(pool)
}
