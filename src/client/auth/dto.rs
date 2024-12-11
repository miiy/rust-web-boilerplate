use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: u32,
    pub user: AuthenticatedUser,
}

#[derive(Debug, Deserialize)]
pub struct AuthenticatedUser {
    pub name: String,
}
