use actix_session::storage::SessionStore;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use std::error::Error;

pub fn session<T>(store: T) -> SessionMiddleware<T>
where
    T: SessionStore + Clone,
{
    let secret_key = Key::generate();
    SessionMiddleware::builder(store.clone(), secret_key.clone())
        .cookie_name("SESSIONID".to_string())
        .build()
}

pub async fn redis_store(redis_url: String) -> Result<RedisSessionStore, Box<dyn Error>> {
    RedisSessionStore::new(redis_url)
        .await
        .map_err(|e| e.into())
}
