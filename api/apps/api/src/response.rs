use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::Value;

use crate::error::ApiError;
use crate::http_log::RequestContext;

#[derive(Debug, Serialize)]
pub struct Envelope {
    pub success: bool,
    pub data: Option<Value>,
    pub error: Option<ErrorPayload>,
    pub meta: Meta,
}

#[derive(Debug, Serialize)]
pub struct ErrorPayload {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct Meta {
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "traceId")]
    pub trace_id: String,
    #[serde(rename = "spanId")]
    pub span_id: String,
    pub timestamp: String,
    #[serde(rename = "durationMs")]
    pub duration_ms: f64,
}

impl Meta {
    pub fn from_context(ctx: &RequestContext) -> Self {
        Self {
            request_id: ctx.request_id.clone(),
            trace_id: ctx.trace_id.clone(),
            span_id: ctx.span_id.clone(),
            timestamp: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            duration_ms: ctx.started.elapsed().as_secs_f64() * 1000.0,
        }
    }
}

pub fn ok<T: Serialize>(ctx: &RequestContext, data: T) -> Response {
    let envelope = Envelope {
        success: true,
        data: Some(serde_json::to_value(data).unwrap_or(Value::Null)),
        error: None,
        meta: Meta::from_context(ctx),
    };
    Json(envelope).into_response()
}

pub fn err(ctx: &RequestContext, error: ApiError) -> Response {
    error.log_if_unexpected();
    let status = error.status();
    let envelope = Envelope {
        success: false,
        data: None,
        error: Some(ErrorPayload { code: error.code().to_string(), message: error.to_string() }),
        meta: Meta::from_context(ctx),
    };
    (status, Json(envelope)).into_response()
}

pub fn respond<T: Serialize>(ctx: &RequestContext, result: Result<T, ApiError>) -> Response {
    match result {
        Ok(data) => ok(ctx, data),
        Err(error) => err(ctx, error),
    }
}
