use serde::{Deserialize, Serialize};
use crate::api::jwt::AuthenticatedUser;

// register
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub id: u64,
}

// login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}


#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: u32,
    pub user: AuthenticatedUser,
}
