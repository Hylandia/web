use jsonwebtoken::{decode, decode_header, Algorithm, Validation};
use url::Url;

use super::claims::{IdTokenClaims, TokenErrorResponse, TokenResponse};
use super::discovery::DiscoveryCache;

pub struct HytaleOidc {
    http: reqwest::Client,
    discovery: DiscoveryCache,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    scopes: String,
}

pub struct ExchangedIdentity {
    pub sub: String,
    pub profile: Option<super::claims::HytaleProfile>,
    pub scope: Option<String>,
    pub access_token: String,
}

impl HytaleOidc {
    pub fn new(issuer: &str, client_id: &str, client_secret: &str, redirect_uri: &str, scopes: &str) -> Self {
        let http = reqwest::Client::new();
        Self {
            discovery: DiscoveryCache::new(http.clone(), issuer.to_string()),
            http,
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            scopes: scopes.to_string(),
        }
    }

    pub async fn authorize_url(&self, state: &str, nonce: &str, code_challenge: &str) -> anyhow::Result<String> {
        let document = self.discovery.document().await?;
        let mut url = Url::parse(&document.authorization_endpoint)?;
        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("scope", &self.scopes)
            .append_pair("state", state)
            .append_pair("nonce", nonce)
            .append_pair("code_challenge", code_challenge)
            .append_pair("code_challenge_method", "S256");
        Ok(url.to_string())
    }

    /// Exchanges an authorization code for tokens, then validates the
    /// `id_token` (signature via JWKS, `iss`, `aud`, `exp`, `nonce`) and
    /// extracts the claims we care about.
    pub async fn exchange_and_verify(
        &self,
        code: &str,
        code_verifier: &str,
        expected_nonce: &str,
    ) -> anyhow::Result<ExchangedIdentity> {
        let document = self.discovery.document().await?;

        let response = self
            .http
            .post(&document.token_endpoint)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", &self.redirect_uri),
                ("code_verifier", code_verifier),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let body: TokenErrorResponse = response
                .json()
                .await
                .unwrap_or(TokenErrorResponse { error: "unknown_error".into(), error_description: None });
            anyhow::bail!("{}: {}", body.error, body.error_description.unwrap_or_default());
        }

        let token_response: TokenResponse = response.json().await?;
        let claims = self.verify_id_token(&token_response.id_token, expected_nonce).await?;

        Ok(ExchangedIdentity {
            sub: claims.sub,
            profile: claims.profile,
            scope: token_response.scope,
            access_token: token_response.access_token,
        })
    }

    async fn verify_id_token(&self, id_token: &str, expected_nonce: &str) -> anyhow::Result<IdTokenClaims> {
        let header = decode_header(id_token)?;
        let kid = header.kid.ok_or_else(|| anyhow::anyhow!("id_token missing kid"))?;
        let key = self.discovery.decoding_key(&kid).await?;

        let document = self.discovery.document().await?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[&document.issuer]);
        validation.set_audience(&[&self.client_id]);

        let claims = decode::<IdTokenClaims>(id_token, &key, &validation)?.claims;

        if claims.nonce.as_deref() != Some(expected_nonce) {
            anyhow::bail!("id_token nonce mismatch");
        }

        Ok(claims)
    }
}
