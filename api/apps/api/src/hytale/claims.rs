use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HytaleProfile {
    pub username: String,
}

/// `iss`/`aud`/`exp` are intentionally not modeled here: `jsonwebtoken`
/// validates those against the raw token payload directly (per
/// `Validation::set_issuer`/`set_audience`), independent of this struct.
#[derive(Debug, Deserialize)]
pub struct IdTokenClaims {
    pub sub: String,
    pub nonce: Option<String>,
    pub profile: Option<HytaleProfile>,
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub scope: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenErrorResponse {
    pub error: String,
    pub error_description: Option<String>,
}
