use std::time::Instant;

use axum::body::Body;
use axum::http::{HeaderValue, Request, Response};
use axum::middleware::Next;
use rand::RngCore;
use uuid::Uuid;

pub const REQUEST_ID_HEADER: &str = "x-request-id";

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub request_id: String,
    pub trace_id: String,
    pub span_id: String,
    pub started: Instant,
}

impl RequestContext {
    pub fn standalone() -> Self {
        Self::generate(None)
    }

    fn generate(incoming_request_id: Option<String>) -> Self {
        Self {
            request_id: incoming_request_id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            trace_id: random_hex(16),
            span_id: random_hex(8),
            started: Instant::now(),
        }
    }
}

fn random_hex(bytes: usize) -> String {
    let mut buf = vec![0u8; bytes];
    rand::thread_rng().fill_bytes(&mut buf);
    buf.iter().map(|b| format!("{b:02x}")).collect()
}

pub async fn context(mut request: Request<Body>, next: Next) -> Response<Body> {
    let incoming_id =
        request.headers().get(REQUEST_ID_HEADER).and_then(|v| v.to_str().ok()).map(str::to_string);

    let ctx = RequestContext::generate(incoming_id);
    let request_id = ctx.request_id.clone();
    request.extensions_mut().insert(ctx);

    let mut response = next.run(request).await;

    if let Ok(value) = HeaderValue::from_str(&request_id) {
        response.headers_mut().insert(REQUEST_ID_HEADER, value);
    }

    response
}

pub async fn access_log(request: Request<Body>, next: Next) -> Response<Body> {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let started = Instant::now();

    let response = next.run(request).await;

    let request_id =
        response.headers().get(REQUEST_ID_HEADER).and_then(|v| v.to_str().ok()).unwrap_or("unknown").to_string();
    let status = response.status().as_u16();
    let latency_ms = started.elapsed().as_millis() as u64;

    tracing::info!(request_id = %request_id, method = %method, path = %path, status, latency_ms, "request completed");

    response
}
