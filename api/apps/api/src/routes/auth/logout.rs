use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use axum_extra::extract::CookieJar;

use crate::cookies;
use crate::http_log::RequestContext;
use crate::response;
use crate::state::AppState;

use super::session::hash_refresh_token;

pub async fn logout(State(state): State<AppState>, Extension(ctx): Extension<RequestContext>, jar: CookieJar) -> Response {
    if let Some(raw) = jar.get(cookies::REFRESH_COOKIE).map(|c| c.value().to_string()) {
        if let Ok(mut conn) = state.db.get().await {
            if let Ok(Some(session)) = db::sessions::find_active_by_hash(&mut conn, &hash_refresh_token(&raw)).await {
                let _ = db::sessions::revoke(&mut conn, session.id).await;
            }
        }
    }

    let jar = jar
        .add(cookies::clear(&state.config, cookies::ACCESS_COOKIE, "/"))
        .add(cookies::clear(&state.config, cookies::REFRESH_COOKIE, "/auth"));

    (jar, response::ok(&ctx, serde_json::json!({}))).into_response()
}
