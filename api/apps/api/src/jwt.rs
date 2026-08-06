use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use p256::elliptic_curve::sec1::ToEncodedPoint;
use p256::pkcs8::DecodePublicKey;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,
    pub iss: String,
    pub aud: String,
    pub iat: i64,
    pub exp: i64,
    pub jti: String,
}

pub struct JwtKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
    issuer: String,
    audience: String,
    ttl_secs: i64,
    kid: String,
    jwk: serde_json::Value,
}

impl JwtKeys {
    pub fn load(private_pem: &str, public_pem: &str, issuer: &str, audience: &str, ttl_secs: i64) -> anyhow::Result<Self> {
        let encoding = EncodingKey::from_ec_pem(private_pem.as_bytes())?;
        let decoding = DecodingKey::from_ec_pem(public_pem.as_bytes())?;

        let kid = URL_SAFE_NO_PAD.encode(Sha256::digest(public_pem.as_bytes()))[..16].to_string();
        let jwk = public_key_jwk(public_pem, &kid)?;

        Ok(Self { encoding, decoding, issuer: issuer.to_string(), audience: audience.to_string(), ttl_secs, kid, jwk })
    }

    pub fn mint_access(&self, user_id: Uuid) -> anyhow::Result<(String, DateTime<Utc>)> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.ttl_secs);
        let claims = AccessClaims {
            sub: user_id.to_string(),
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
            jti: Uuid::new_v4().to_string(),
        };

        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(self.kid.clone());

        let token = encode(&header, &claims, &self.encoding)?;
        Ok((token, exp))
    }

    pub fn verify_access(&self, token: &str) -> Result<AccessClaims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::ES256);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);
        decode::<AccessClaims>(token, &self.decoding, &validation).map(|data| data.claims)
    }

    pub fn jwks_document(&self) -> serde_json::Value {
        serde_json::json!({ "keys": [self.jwk] })
    }
}

fn public_key_jwk(public_pem: &str, kid: &str) -> anyhow::Result<serde_json::Value> {
    let key = p256::PublicKey::from_public_key_pem(public_pem)?;
    let point = key.to_encoded_point(false);
    let x = URL_SAFE_NO_PAD.encode(point.x().ok_or_else(|| anyhow::anyhow!("missing x coordinate"))?);
    let y = URL_SAFE_NO_PAD.encode(point.y().ok_or_else(|| anyhow::anyhow!("missing y coordinate"))?);

    Ok(serde_json::json!({
        "kty": "EC",
        "crv": "P-256",
        "alg": "ES256",
        "use": "sig",
        "kid": kid,
        "x": x,
        "y": y,
    }))
}
