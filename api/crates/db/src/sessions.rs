use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use ipnetwork::IpNetwork;
use uuid::Uuid;

use crate::models::{NewSession, Session};
use crate::schema::sessions;

pub async fn create(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    refresh_token_hash: &str,
    user_agent: Option<&str>,
    ip_address: Option<IpNetwork>,
    expires_at: DateTime<Utc>,
) -> QueryResult<Session> {
    diesel::insert_into(sessions::table)
        .values(NewSession {
            user_id,
            refresh_token_hash,
            user_agent,
            ip_address,
            expires_at,
        })
        .get_result(conn)
        .await
}

/// Looks up a session by its refresh token hash, only if it hasn't been
/// revoked or expired. Callers should treat "not found" and "expired /
/// revoked" identically (reject the refresh) rather than distinguishing them.
pub async fn find_active_by_hash(
    conn: &mut AsyncPgConnection,
    hash: &str,
) -> QueryResult<Option<Session>> {
    sessions::table
        .filter(sessions::refresh_token_hash.eq(hash))
        .filter(sessions::revoked_at.is_null())
        .filter(sessions::expires_at.gt(diesel::dsl::now))
        .first(conn)
        .await
        .optional()
}

/// Rotates the refresh token in place on the same session row (no token
/// family / reuse-detection chain yet, see README for the caveat, let's just hope user never changes their profile info (: ).
pub async fn rotate(
    conn: &mut AsyncPgConnection,
    id: Uuid,
    new_hash: &str,
    new_expires_at: DateTime<Utc>,
) -> QueryResult<Session> {
    diesel::update(sessions::table.find(id))
        .set((
            sessions::refresh_token_hash.eq(new_hash),
            sessions::expires_at.eq(new_expires_at),
        ))
        .get_result(conn)
        .await
}

pub async fn revoke(conn: &mut AsyncPgConnection, id: Uuid) -> QueryResult<usize> {
    diesel::update(sessions::table.find(id))
        .set(sessions::revoked_at.eq(diesel::dsl::now))
        .execute(conn)
        .await
}

pub async fn revoke_all_for_user(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
) -> QueryResult<usize> {
    diesel::update(
        sessions::table
            .filter(sessions::user_id.eq(user_id))
            .filter(sessions::revoked_at.is_null()),
    )
    .set(sessions::revoked_at.eq(diesel::dsl::now))
    .execute(conn)
    .await
}
