use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::models::{NewOAuthAccount, OAuthAccount};
use crate::schema::oauth_accounts;

/// Records the Hytale tokens obtained at login. There is no refresh grant
/// on Hytale's side, so `access_token`/`expires_at` are a snapshot of the
/// last interactive sign-in, not something kept fresh in the background.
#[allow(clippy::too_many_arguments)]
pub async fn upsert(
    conn: &mut AsyncPgConnection,
    user_id: Uuid,
    provider: &str,
    provider_user_id: &str,
    access_token: &str,
    refresh_token: Option<&str>,
    expires_at: Option<DateTime<Utc>>,
    scope: Option<&str>,
) -> QueryResult<OAuthAccount> {
    diesel::insert_into(oauth_accounts::table)
        .values(NewOAuthAccount {
            user_id,
            provider,
            provider_user_id,
            access_token,
            refresh_token,
            expires_at,
            scope,
        })
        .on_conflict((oauth_accounts::provider, oauth_accounts::provider_user_id))
        .do_update()
        .set((
            oauth_accounts::access_token.eq(access_token),
            oauth_accounts::refresh_token.eq(refresh_token),
            oauth_accounts::expires_at.eq(expires_at),
            oauth_accounts::scope.eq(scope),
            oauth_accounts::updated_at.eq(diesel::dsl::now),
        ))
        .get_result(conn)
        .await
}
