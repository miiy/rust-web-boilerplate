use actix_session::storage::{RedisSessionStore, SessionStore};
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use base64::prelude::*;
use std::error::Error;
use std::str::FromStr;

pub fn session<T>(store: T, cookie_name: String, secret_key: SecretKey) -> SessionMiddleware<T>
where
    T: SessionStore + Clone,
{
    SessionMiddleware::builder(store.clone(), secret_key.key())
        .cookie_name(cookie_name)
        .build()
}

pub async fn redis_store(redis_url: String) -> Result<RedisSessionStore, Box<dyn Error>> {
    RedisSessionStore::new(redis_url)
        .await
        .map_err(|e| e.into())
}

#[derive(Clone)]
pub struct SecretKey(Key);

impl FromStr for SecretKey {
    type Err = base64::DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = BASE64_STANDARD.decode(s)?;
        let key = Key::from(&v);
        Ok(Self(key))
    }
}

impl SecretKey {
    pub fn new() -> Self {
        let key = Key::generate();
        Self(key)
    }

    pub fn key(&self) -> Key {
        self.0.clone()
    }

    pub fn encode_base64(&self) -> String {
        BASE64_STANDARD.encode(self.0.master())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_secret_key() {
        let key = SecretKey::new();
        assert!(key.encode_base64().len() > 0);

        let key_str = "dpp7QFH3QR41Ims6mIjdTysAtkM9bOoN3gJAaF0X5HqbKdGdVhWQcdMcJTtqjpUAL4EolObWxhG9s8uN2e32vA==";
        let key = SecretKey::from_str(key_str).unwrap();
        assert_eq!(key.encode_base64(), key_str);
    }
}
