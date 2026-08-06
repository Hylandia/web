use axum_extra::extract::cookie::{Cookie, SameSite};
use time::Duration;

use crate::config::Config;

pub const OAUTH_COOKIE: &str = "hytale_oauth";
pub const ACCESS_COOKIE: &str = "access_token";
pub const REFRESH_COOKIE: &str = "refresh_token";

fn builder<'a>(config: &Config, name: &'a str, value: String, path: &'a str) -> Cookie<'a> {
    let mut cookie = Cookie::new(name, value);
    cookie.set_http_only(true);
    cookie.set_secure(config.cookie_secure);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path(path.to_string());
    if !config.cookie_domain.is_empty() {
        cookie.set_domain(config.cookie_domain.clone());
    }
    cookie
}

/// Holds `state`/`nonce`/`code_verifier`/`redirect_after` between the
/// `/auth/hytale/login` redirect and the `/auth/hytale/callback` round trip.
/// Scoped to `/auth/hytale` and expires quickly since it's only alive for
/// the duration of the redirect to Hytale and back.
pub fn oauth_state_cookie(config: &Config, value: String) -> Cookie<'static> {
    let mut cookie = builder(config, OAUTH_COOKIE, value, "/auth/hytale").into_owned();
    cookie.set_max_age(Duration::minutes(10));
    cookie
}

pub fn access_cookie(config: &Config, token: String, ttl_secs: i64) -> Cookie<'static> {
    let mut cookie = builder(config, ACCESS_COOKIE, token, "/").into_owned();
    cookie.set_max_age(Duration::seconds(ttl_secs));
    cookie
}

pub fn refresh_cookie(config: &Config, token: String, ttl_secs: i64) -> Cookie<'static> {
    let mut cookie = builder(config, REFRESH_COOKIE, token, "/auth").into_owned();
    cookie.set_same_site(SameSite::Strict);
    cookie.set_max_age(Duration::seconds(ttl_secs));
    cookie
}

pub fn clear(config: &Config, name: &'static str, path: &'static str) -> Cookie<'static> {
    let mut cookie = builder(config, name, String::new(), path).into_owned();
    cookie.set_max_age(Duration::seconds(0));
    cookie
}
