use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OauthState {
    pub state: String,
    pub nonce: String,
    pub verifier: String,
    pub redirect_path: String,
}

impl OauthState {
    pub fn encode(&self) -> String {
        URL_SAFE_NO_PAD.encode(serde_json::to_vec(self).expect("OauthState is always serializable"))
    }

    pub fn decode(value: &str) -> Option<Self> {
        let bytes = URL_SAFE_NO_PAD.decode(value).ok()?;
        serde_json::from_slice(&bytes).ok()
    }
}

/// Only relative, single-leading-slash paths are allowed — otherwise a
/// crafted `redirect` query param could bounce the browser (with our
/// session cookies already set) off to an attacker-controlled origin.
pub fn sanitize_redirect_path(raw: Option<&str>) -> String {
    match raw {
        Some(path) if path.starts_with('/') && !path.starts_with("//") && !path.contains("://") => path.to_string(),
        _ => "/".to_string(),
    }
}
