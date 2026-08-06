use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::cookies;
use crate::error::ApiError;
use crate::state::AppState;

use super::oauth_state::OauthState;
use super::session::issue_new_session;

#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

pub async fn callback(
    State(state): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    Query(query): Query<CallbackQuery>,
) -> Response {
    let clear_oauth_cookie = cookies::clear(&state.config, cookies::OAUTH_COOKIE, "/auth/hytale");
    let jar = jar.add(clear_oauth_cookie);

    match handle(&state, &headers, &jar, query).await {
        Ok((redirect, tokens)) => (jar.add(tokens.access_cookie).add(tokens.refresh_cookie), redirect).into_response(),
        Err(error) => (jar, error).into_response(),
    }
}

async fn handle(
    state: &AppState,
    headers: &HeaderMap,
    jar: &CookieJar,
    query: CallbackQuery,
) -> Result<(Redirect, super::session::IssuedTokens), ApiError> {
    if let Some(error) = query.error {
        return Err(ApiError::HytaleAuth(format!("{error}: {}", query.error_description.unwrap_or_default())));
    }

    let saved = jar
        .get(cookies::OAUTH_COOKIE)
        .and_then(|c| OauthState::decode(c.value()))
        .ok_or_else(|| ApiError::BadRequest("missing or expired oauth state".into()))?;

    let code = query.code.ok_or_else(|| ApiError::BadRequest("missing code".into()))?;
    let returned_state = query.state.ok_or_else(|| ApiError::BadRequest("missing state".into()))?;
    if returned_state != saved.state {
        return Err(ApiError::BadRequest("state mismatch".into()));
    }

    let identity = state
        .hytale
        .exchange_and_verify(&code, &saved.verifier, &saved.nonce)
        .await
        .map_err(|e| ApiError::HytaleAuth(e.to_string()))?;

    let username = identity.profile.as_ref().map(|p| p.username.clone()).unwrap_or_else(|| identity.sub.clone());

    let mut conn = state.db.get().await?;
    let user = db::users::upsert_from_hytale(&mut conn, &identity.sub, &username).await?;

    let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);
    db::oauth_accounts::upsert(
        &mut conn,
        user.id,
        "hytale",
        &identity.sub,
        &identity.access_token,
        None,
        Some(expires_at),
        identity.scope.as_deref(),
    )
    .await?;

    let user_agent = headers.get(axum::http::header::USER_AGENT).and_then(|v| v.to_str().ok());
    let ip = client_ip(headers);

    let tokens = issue_new_session(&mut conn, state, user.id, user_agent, ip).await?;
    let redirect = Redirect::to(&format!("{}{}", state.config.frontend_url, saved.redirect_path));
    Ok((redirect, tokens))
}

/// Fly's edge terminates TLS and forwards the real client address in
/// `Fly-Client-IP`; `X-Forwarded-For` is a fallback for local/other environments.
fn client_ip(headers: &HeaderMap) -> Option<ipnetwork::IpNetwork> {
    let raw = headers
        .get("fly-client-ip")
        .and_then(|v| v.to_str().ok())
        .or_else(|| headers.get("x-forwarded-for").and_then(|v| v.to_str().ok())?.split(',').next())?;
    raw.trim().parse().ok()
}
