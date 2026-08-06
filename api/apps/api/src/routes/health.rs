use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use diesel_async::RunQueryDsl;
use serde::Serialize;

use crate::state::AppState;

#[derive(Debug, Serialize)]
struct HealthzBody {
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct ReadyzBody {
    ready: bool,
    database: bool,
}

pub async fn healthz() -> impl IntoResponse {
    Json(HealthzBody { status: "ok" })
}

pub async fn readyz(State(state): State<AppState>) -> Response {
    let database = match state.db.get().await {
        Ok(mut conn) => diesel::sql_query("SELECT 1").execute(&mut conn).await.is_ok(),
        Err(_) => false,
    };

    let status = if database { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    (status, Json(ReadyzBody { ready: database, database })).into_response()
}
