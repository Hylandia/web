//! Environment-driven configuration, loaded once at startup.

use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    Text,
    Json,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub log_format: LogFormat,
    pub database_url: String,

    /// `https://connect.accounts.hytale.com` — overridable for testing against a mock issuer.
    pub hytale_issuer: String,
    pub hytale_client_id: String,
    pub hytale_client_secret: String,
    /// Must exactly match one of the URIs registered for this client.
    pub hytale_redirect_uri: String,
    /// Space-separated scope string sent on every `/oauth2/auth` request.
    pub hytale_scopes: String,

    /// ES256 private key, PEM (SEC1 or PKCS8).
    pub jwt_private_key_pem: String,
    /// ES256 public key, PEM.
    pub jwt_public_key_pem: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub access_token_ttl_secs: i64,
    pub refresh_token_ttl_secs: i64,

    /// Origin the browser is redirected back to after login (also the only
    /// origin `redirect_after` is allowed to point at — see routes/auth/login.rs).
    pub frontend_url: String,
    /// `Domain=` attribute for session cookies. Empty means host-only.
    pub cookie_domain: String,
    /// Set false only for plain-http local dev; cookies otherwise require `Secure`.
    pub cookie_secure: bool,

    /// Optional. When set, errors are reported to Sentry.
    pub sentry_dsn: Option<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let port = env::var("PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(8080);

        let log_format = match env::var("LOG_FORMAT").as_deref() {
            Ok("json") => LogFormat::Json,
            _ => LogFormat::Text,
        };

        let database_url = require_env("DATABASE_URL")?;

        let hytale_issuer =
            env::var("HYTALE_ISSUER").unwrap_or_else(|_| "https://connect.accounts.hytale.com".to_string());
        let hytale_client_id = require_env("HYTALE_CLIENT_ID")?;
        let hytale_client_secret = require_env("HYTALE_CLIENT_SECRET")?;
        let hytale_redirect_uri = require_env("HYTALE_REDIRECT_URI")?;
        let hytale_scopes = env::var("HYTALE_SCOPES").unwrap_or_else(|_| "openid hytale:profile".to_string());

        let jwt_private_key_pem = require_env("JWT_PRIVATE_KEY_PEM")?;
        let jwt_public_key_pem = require_env("JWT_PUBLIC_KEY_PEM")?;
        let jwt_issuer = env::var("JWT_ISSUER").unwrap_or_else(|_| "hylandia-web-api".to_string());
        let jwt_audience = env::var("JWT_AUDIENCE").unwrap_or_else(|_| "hylandia".to_string());
        let access_token_ttl_secs =
            env::var("ACCESS_TOKEN_TTL_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(900);
        let refresh_token_ttl_secs = env::var("REFRESH_TOKEN_TTL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60 * 60 * 24 * 30);

        let frontend_url = require_env("FRONTEND_URL")?.trim_end_matches('/').to_string();
        let cookie_domain = env::var("COOKIE_DOMAIN").unwrap_or_default();
        let cookie_secure = env::var("COOKIE_SECURE").map(|v| !v.eq_ignore_ascii_case("false")).unwrap_or(true);

        Ok(Self {
            port,
            log_format,
            database_url,
            hytale_issuer,
            hytale_client_id,
            hytale_client_secret,
            hytale_redirect_uri,
            hytale_scopes,
            jwt_private_key_pem,
            jwt_public_key_pem,
            jwt_issuer,
            jwt_audience,
            access_token_ttl_secs,
            refresh_token_ttl_secs,
            frontend_url,
            cookie_domain,
            cookie_secure,
            sentry_dsn: env::var("SENTRY_DSN").ok().filter(|v| !v.trim().is_empty()),
        })
    }
}

fn require_env(key: &str) -> anyhow::Result<String> {
    env::var(key).ok().filter(|v| !v.trim().is_empty()).ok_or_else(|| anyhow::anyhow!("{key} must be set"))
}
