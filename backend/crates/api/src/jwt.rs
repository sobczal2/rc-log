use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub username: String,
    pub exp: u64,
}

const TOKEN_EXPIRY_SECS: u64 = 86_400; // 24 hours

pub fn create_token(
    claims: &JwtClaims,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&Header::default(), claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn verify_token(token: &str, secret: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
    let data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

pub fn new_claims(id: Uuid, username: String) -> JwtClaims {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before Unix epoch")
        .as_secs()
        + TOKEN_EXPIRY_SECS;
    JwtClaims { sub: id, username, exp }
}
