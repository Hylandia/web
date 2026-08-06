use axum::extract::State;
use axum::response::Response;
use axum::Extension;
use serde::Serialize;

use crate::error::ApiError;
use crate::http_log::RequestContext;
use crate::response::respond;
use crate::state::AppState;

use super::extract::AuthUser;

#[derive(Debug, Serialize)]
struct MeResponse {
    id: uuid::Uuid,
    username: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    #[serde(rename = "avatarUrl")]
    avatar_url: Option<String>,
    email: Option<String>,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
}

pub async fn me(State(state): State<AppState>, Extension(ctx): Extension<RequestContext>, AuthUser(user_id): AuthUser) -> Response {
    let result = fetch(&state, user_id).await;
    respond(&ctx, result)
}

async fn fetch(state: &AppState, user_id: uuid::Uuid) -> Result<MeResponse, ApiError> {
    let mut conn = state.db.get().await?;
    let user = db::users::find_by_id(&mut conn, user_id).await?.ok_or(ApiError::NotFound)?;

    Ok(MeResponse {
        id: user.id,
        username: user.username,
        display_name: user.display_name,
        avatar_url: user.avatar_url,
        email: user.email,
        email_verified: user.email_verified,
    })
}
