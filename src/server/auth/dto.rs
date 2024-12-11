use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {}
