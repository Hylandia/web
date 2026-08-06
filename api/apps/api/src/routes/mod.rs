mod auth;
mod health;
mod jwks;

use axum::http::{header, HeaderValue, Method};
use axum::routing::get;
use axum::Router;
use tower_http::cors::CorsLayer;

use crate::state::AppState;

pub fn build(state: AppState) -> Router {
    let origin = HeaderValue::from_str(&state.config.frontend_url).expect("FRONTEND_URL is a valid header value");

    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    Router::new()
        .route("/healthz", get(health::healthz))
        .route("/readyz", get(health::readyz))
        .route("/.well-known/jwks.json", get(jwks::jwks))
        .merge(auth::router())
        .layer(cors)
        .with_state(state)
}
