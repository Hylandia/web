use std::sync::Arc;

use db::DbPool;

use crate::config::Config;
use crate::hytale::HytaleOidc;
use crate::jwt::JwtKeys;

pub struct AppStateInner {
    pub db: DbPool,
    pub hytale: HytaleOidc,
    pub jwt: JwtKeys,
    pub config: Config,
}

pub type AppState = Arc<AppStateInner>;
