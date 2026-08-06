use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::cookies::ACCESS_COOKIE;
use crate::error::ApiError;
use crate::state::AppState;

pub struct AuthUser(pub Uuid);

#[axum::async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await.expect("CookieJar extraction is infallible");

        let token = jar
            .get(ACCESS_COOKIE)
            .map(|c| c.value().to_string())
            .or_else(|| {
                parts
                    .headers
                    .get(AUTHORIZATION)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.strip_prefix("Bearer "))
                    .map(str::to_string)
            })
            .ok_or(ApiError::Unauthorized)?;

        let claims = state.jwt.verify_access(&token).map_err(|_| ApiError::Unauthorized)?;
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| ApiError::Unauthorized)?;
        Ok(AuthUser(user_id))
    }
}
