use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::cookies;
use crate::error::ApiError;
use crate::hytale::pkce::{random_urlsafe, Pkce};
use crate::state::AppState;

use super::oauth_state::{sanitize_redirect_path, OauthState};

#[derive(Debug, Deserialize)]
pub struct LoginQuery {
    redirect: Option<String>,
}

pub async fn login(State(state): State<AppState>, jar: CookieJar, Query(query): Query<LoginQuery>) -> Response {
    match build_redirect(&state, query).await {
        Ok((redirect, cookie)) => (jar.add(cookie), redirect).into_response(),
        Err(error) => error.into_response(),
    }
}

async fn build_redirect(state: &AppState, query: LoginQuery) -> Result<(Redirect, axum_extra::extract::cookie::Cookie<'static>), ApiError> {
    let pkce = Pkce::generate();
    let oauth_state = random_urlsafe(16);
    let nonce = random_urlsafe(16);
    let redirect_path = sanitize_redirect_path(query.redirect.as_deref());

    let authorize_url = state
        .hytale
        .authorize_url(&oauth_state, &nonce, &pkce.challenge)
        .await
        .map_err(ApiError::Internal)?;

    let cookie_value = OauthState { state: oauth_state, nonce, verifier: pkce.verifier, redirect_path }.encode();
    let cookie = cookies::oauth_state_cookie(&state.config, cookie_value);

    Ok((Redirect::to(&authorize_url), cookie))
}
