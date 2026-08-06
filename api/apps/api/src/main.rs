mod config;
mod cookies;
mod error;
mod http_log;
mod hytale;
mod jwt;
mod response;
mod routes;
mod state;
mod telemetry;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::middleware;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

use crate::config::{Config, LogFormat};
use crate::hytale::HytaleOidc;
use crate::jwt::JwtKeys;
use crate::state::{AppState, AppStateInner};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;
    let _sentry = telemetry::init(&config);
    init_tracing(&config);

    tracing::info!(port = config.port, "starting hylandia web api");

    db::migrations::run_pending_migrations(&config.database_url)?;
    tracing::info!("database migrations applied");

    let pool = db::build_pool(&config.database_url).await?;

    let hytale = HytaleOidc::new(
        &config.hytale_issuer,
        &config.hytale_client_id,
        &config.hytale_client_secret,
        &config.hytale_redirect_uri,
        &config.hytale_scopes,
    );

    let jwt = JwtKeys::load(
        &config.jwt_private_key_pem,
        &config.jwt_public_key_pem,
        &config.jwt_issuer,
        &config.jwt_audience,
        config.access_token_ttl_secs,
    )?;

    let port = config.port;
    let state: AppState = Arc::new(AppStateInner { db: pool, hytale, jwt, config });

    let app = routes::build(state)
        .layer(middleware::from_fn(http_log::access_log))
        .layer(middleware::from_fn(http_log::context));

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
    tracing::info!(%addr, "listening");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).with_graceful_shutdown(shutdown_signal()).await?;

    Ok(())
}

fn init_tracing(config: &Config) {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("api=info,tower_http=info"));

    let fmt_layer = match config.log_format {
        LogFormat::Json => tracing_subscriber::fmt::layer().json().boxed(),
        LogFormat::Text => tracing_subscriber::fmt::layer().boxed(),
    };

    let registry = tracing_subscriber::registry().with(env_filter).with(fmt_layer);
    if config.sentry_dsn.as_ref().is_some_and(|d| !d.is_empty()) {
        registry.with(sentry_tracing::layer()).init();
    } else {
        registry.init();
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("failed to install CTRL+C handler");
    tracing::info!("shutdown signal received");
}
