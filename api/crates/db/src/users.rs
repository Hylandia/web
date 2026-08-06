use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::models::{NewUser, User};
use crate::schema::users;

/// Inserts a user on first sign-in, or updates `username` if the player
/// picked a different profile since last time. `hytale_id` is the OIDC
/// `sub` — stable per-application and unaffected by profile switches, so
/// it's the identity key rather than `profile.uuid`.
pub async fn upsert_from_hytale(
    conn: &mut AsyncPgConnection,
    hytale_id: &str,
    username: &str,
) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(NewUser { hytale_id, username })
        .on_conflict(users::hytale_id)
        .do_update()
        .set((users::username.eq(username), users::updated_at.eq(diesel::dsl::now)))
        .get_result(conn)
        .await
}

pub async fn find_by_id(conn: &mut AsyncPgConnection, id: Uuid) -> QueryResult<Option<User>> {
    users::table.find(id).first(conn).await.optional()
}
