use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct AuthenticatedUser {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct JWT {
    pub secret: String,
    pub expires_in: u32,
}

pub enum JWTError {
    DecodeError { source: jsonwebtoken::errors::Error },
}

impl JWT {
    pub fn new(secret: String, expires_in: u32) -> Self {
        Self { secret, expires_in }
    }

    pub fn create_claims(&self, sub: String) -> Claims {
        Claims {
            sub,
            company: "rust_web".to_string(),
            exp: (OffsetDateTime::now_utc() + Duration::hours(12)).unix_timestamp(),
        }
    }

    pub fn encode(&self, claims: &Claims) -> jsonwebtoken::errors::Result<String> {
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    pub fn decode(&self, token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_and_decode() {
        let jwt = JWT::new("secret".to_string(), 12);
        let claims = jwt.create_claims("test".to_string());
        let token = jwt.encode(&claims).unwrap();
        let decoded = jwt.decode(&token).unwrap();
        assert_eq!(decoded.claims.sub, "test");
    }
}
