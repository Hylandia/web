//! Typed API errors. Handlers return `Result<T, ApiError>`; `response::respond`
//! renders either branch into the same envelope shape via `Meta::from_context`.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("invalid request: {0}")]
    BadRequest(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("not found")]
    NotFound,

    /// Surfaced from Hytale's authorization/token endpoints, or from failed
    /// id_token validation. Always the user's fault or a stale link, never
    /// worth a 500 or a Sentry-grade log line.
    #[error("hytale sign-in failed: {0}")]
    HytaleAuth(String),

    #[error("database error: {0}")]
    Database(#[from] diesel::result::Error),

    #[error("database pool error: {0}")]
    Pool(#[from] diesel_async::pooled_connection::bb8::RunError),

    #[error("upstream request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("token error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl ApiError {
    pub fn status(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::HytaleAuth(_) => StatusCode::BAD_REQUEST,
            ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Pool(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Http(_) => StatusCode::BAD_GATEWAY,
            ApiError::Jwt(_) => StatusCode::UNAUTHORIZED,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            ApiError::BadRequest(_) => "BAD_REQUEST",
            ApiError::Unauthorized => "UNAUTHORIZED",
            ApiError::NotFound => "NOT_FOUND",
            ApiError::HytaleAuth(_) => "HYTALE_AUTH_FAILED",
            ApiError::Database(_) => "DATABASE_ERROR",
            ApiError::Pool(_) => "DATABASE_ERROR",
            ApiError::Http(_) => "UPSTREAM_ERROR",
            ApiError::Jwt(_) => "INVALID_TOKEN",
            ApiError::Internal(_) => "INTERNAL_ERROR",
        }
    }

    pub fn log_if_unexpected(&self) {
        let expected = matches!(
            self,
            ApiError::BadRequest(_) | ApiError::Unauthorized | ApiError::NotFound | ApiError::HytaleAuth(_) | ApiError::Jwt(_)
        );
        if !expected {
            tracing::error!(error = %self, code = self.code(), "request failed");
            sentry::capture_message(&self.to_string(), sentry::Level::Error);
        }
    }
}

/// Used when an error surfaces outside a handler (e.g. extractor
/// rejections), where there's no `RequestContext` to thread through. The
/// envelope still comes out the same shape, just with a fresh request id.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        crate::response::err(&crate::http_log::RequestContext::standalone(), self)
    }
}
