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

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use uuid::Uuid;

    use super::{JwtClaims, create_token, new_claims, verify_token, TOKEN_EXPIRY_SECS};

    const SECRET: &str = "test-secret-key";

    fn make_claims() -> JwtClaims {
        let id = Uuid::new_v4();
        new_claims(id, "testuser".to_string())
    }

    #[test]
    fn create_and_verify_round_trip() {
        let claims = make_claims();
        let token = create_token(&claims, SECRET).expect("token creation should succeed");
        let verified = verify_token(&token, SECRET).expect("verification should succeed");
        assert_eq!(claims.sub, verified.sub);
        assert_eq!(claims.username, verified.username);
        assert_eq!(claims.exp, verified.exp);
    }

    #[test]
    fn verify_with_wrong_secret_fails() {
        let claims = make_claims();
        let token = create_token(&claims, SECRET).unwrap();
        let result = verify_token(&token, "wrong-secret");
        assert!(result.is_err());
    }

    #[test]
    fn verify_malformed_token_fails() {
        let result = verify_token("not.a.valid.jwt", SECRET);
        assert!(result.is_err());
    }

    #[test]
    fn verify_empty_token_fails() {
        let result = verify_token("", SECRET);
        assert!(result.is_err());
    }

    #[test]
    fn new_claims_sets_exp_in_future() {
        let before = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let claims = new_claims(Uuid::new_v4(), "user".to_string());
        let after = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        assert!(claims.exp >= before + TOKEN_EXPIRY_SECS);
        assert!(claims.exp <= after + TOKEN_EXPIRY_SECS);
    }

    #[test]
    fn new_claims_stores_id_and_username() {
        let id = Uuid::new_v4();
        let claims = new_claims(id, "alice".to_string());
        assert_eq!(claims.sub, id);
        assert_eq!(claims.username, "alice");
    }
}
