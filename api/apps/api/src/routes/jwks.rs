use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;

use crate::state::AppState;

pub async fn jwks(State(state): State<AppState>) -> impl IntoResponse {
    Json(state.jwt.jwks_document())
}
