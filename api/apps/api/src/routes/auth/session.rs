use axum_extra::extract::cookie::Cookie;
use chrono::{Duration, Utc};
use db::AsyncPgConnection;
use ipnetwork::IpNetwork;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::cookies;
use crate::error::ApiError;
use crate::hytale::pkce::random_urlsafe;
use crate::state::AppState;

pub struct IssuedTokens {
    pub access_cookie: Cookie<'static>,
    pub refresh_cookie: Cookie<'static>,
}

pub fn hash_refresh_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

pub async fn issue_new_session(
    conn: &mut AsyncPgConnection,
    state: &AppState,
    user_id: Uuid,
    user_agent: Option<&str>,
    ip: Option<IpNetwork>,
) -> Result<IssuedTokens, ApiError> {
    let refresh_token = random_urlsafe(32);
    let expires_at = Utc::now() + Duration::seconds(state.config.refresh_token_ttl_secs);
    db::sessions::create(conn, user_id, &hash_refresh_token(&refresh_token), user_agent, ip, expires_at).await?;
    finish(state, user_id, refresh_token)
}

pub async fn rotate_session(
    conn: &mut AsyncPgConnection,
    state: &AppState,
    session_id: Uuid,
    user_id: Uuid,
) -> Result<IssuedTokens, ApiError> {
    let refresh_token = random_urlsafe(32);
    let expires_at = Utc::now() + Duration::seconds(state.config.refresh_token_ttl_secs);
    db::sessions::rotate(conn, session_id, &hash_refresh_token(&refresh_token), expires_at).await?;
    finish(state, user_id, refresh_token)
}

fn finish(state: &AppState, user_id: Uuid, refresh_token: String) -> Result<IssuedTokens, ApiError> {
    let (access_token, _exp) = state.jwt.mint_access(user_id).map_err(ApiError::Internal)?;
    Ok(IssuedTokens {
        access_cookie: cookies::access_cookie(&state.config, access_token, state.config.access_token_ttl_secs),
        refresh_cookie: cookies::refresh_cookie(&state.config, refresh_token, state.config.refresh_token_ttl_secs),
    })
}
