use std::collections::HashMap;
use std::time::{Duration, Instant};

use jsonwebtoken::DecodingKey;
use serde::Deserialize;
use tokio::sync::RwLock;

const REFRESH_INTERVAL: Duration = Duration::from_secs(60 * 60);

#[derive(Debug, Clone, Deserialize)]
pub struct DiscoveryDocument {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    #[allow(dead_code)]
    pub userinfo_endpoint: String,
    pub jwks_uri: String,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String,
    kty: String,
    n: Option<String>,
    e: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

struct Cached {
    document: DiscoveryDocument,
    keys: HashMap<String, DecodingKey>,
    fetched_at: Instant,
}

/// Discovery document + JWKS, fetched lazily and refreshed hourly (or
/// immediately if a `kid` we haven't seen shows up, in case Hytale rotated
/// keys sooner than that).
pub struct DiscoveryCache {
    http: reqwest::Client,
    issuer: String,
    cached: RwLock<Option<Cached>>,
}

impl DiscoveryCache {
    pub fn new(http: reqwest::Client, issuer: String) -> Self {
        Self { http, issuer, cached: RwLock::new(None) }
    }

    pub async fn document(&self) -> anyhow::Result<DiscoveryDocument> {
        self.ensure_fresh(false).await?;
        Ok(self.cached.read().await.as_ref().expect("just refreshed").document.clone())
    }

    pub async fn decoding_key(&self, kid: &str) -> anyhow::Result<DecodingKey> {
        self.ensure_fresh(false).await?;
        if let Some(key) = self.lookup(kid).await {
            return Ok(key);
        }

        // Key not found, maybe it rotated since our last fetch. Try once more.
        self.ensure_fresh(true).await?;
        self.lookup(kid).await.ok_or_else(|| anyhow::anyhow!("unknown signing key: {kid}"))
    }

    async fn lookup(&self, kid: &str) -> Option<DecodingKey> {
        self.cached.read().await.as_ref().and_then(|c| c.keys.get(kid).cloned())
    }

    async fn ensure_fresh(&self, force: bool) -> anyhow::Result<()> {
        {
            let guard = self.cached.read().await;
            if let Some(cached) = guard.as_ref() {
                if !force && cached.fetched_at.elapsed() < REFRESH_INTERVAL {
                    return Ok(());
                }
            }
        }

        let discovery_url = format!("{}/.well-known/openid-configuration", self.issuer.trim_end_matches('/'));
        let document: DiscoveryDocument = self.http.get(&discovery_url).send().await?.error_for_status()?.json().await?;

        let jwk_set: JwkSet = self.http.get(&document.jwks_uri).send().await?.error_for_status()?.json().await?;

        let mut keys = HashMap::new();
        for jwk in jwk_set.keys {
            if jwk.kty != "RSA" {
                continue;
            }
            let (Some(n), Some(e)) = (jwk.n.as_deref(), jwk.e.as_deref()) else { continue };
            if let Ok(key) = DecodingKey::from_rsa_components(n, e) {
                keys.insert(jwk.kid, key);
            }
        }

        *self.cached.write().await = Some(Cached { document, keys, fetched_at: Instant::now() });
        Ok(())
    }
}
