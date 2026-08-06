mod callback;
pub mod extract;
mod login;
mod logout;
mod me;
mod oauth_state;
mod refresh;
mod session;

use axum::routing::{get, post};
use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/hytale/login", get(login::login))
        .route("/auth/hytale/callback", get(callback::callback))
        .route("/auth/refresh", post(refresh::refresh))
        .route("/auth/logout", post(logout::logout))
        .route("/auth/me", get(me::me))
}
