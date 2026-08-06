use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use axum_extra::extract::CookieJar;

use crate::cookies;
use crate::error::ApiError;
use crate::http_log::RequestContext;
use crate::response;
use crate::state::AppState;

use super::session::{hash_refresh_token, rotate_session};

pub async fn refresh(State(state): State<AppState>, Extension(ctx): Extension<RequestContext>, jar: CookieJar) -> Response {
    match handle(&state, &jar).await {
        Ok(tokens) => {
            let jar = jar.add(tokens.access_cookie).add(tokens.refresh_cookie);
            (jar, response::ok(&ctx, serde_json::json!({}))).into_response()
        }
        Err(error) => {
            let jar = jar
                .add(cookies::clear(&state.config, cookies::ACCESS_COOKIE, "/"))
                .add(cookies::clear(&state.config, cookies::REFRESH_COOKIE, "/auth"));
            (jar, response::err(&ctx, error)).into_response()
        }
    }
}

async fn handle(state: &AppState, jar: &CookieJar) -> Result<super::session::IssuedTokens, ApiError> {
    let raw = jar.get(cookies::REFRESH_COOKIE).map(|c| c.value().to_string()).ok_or(ApiError::Unauthorized)?;

    let mut conn = state.db.get().await?;
    let session = db::sessions::find_active_by_hash(&mut conn, &hash_refresh_token(&raw))
        .await?
        .ok_or(ApiError::Unauthorized)?;

    rotate_session(&mut conn, state, session.id, session.user_id).await
}
