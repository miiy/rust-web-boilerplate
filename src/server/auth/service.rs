use super::dto::*;
use super::error::AuthError;
use crate::client::auth::client::AuthClient;
use crate::client::auth::dto as auth_dto;
use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest};

pub async fn register(
    req: RegisterRequest,
    client: &AuthClient,
) -> Result<RegisterResponse, AuthError> {
    validate_register(&req)?;

    let req = auth_dto::RegisterRequest {
        name: req.name,
        email: req.email,
        password: req.password,
        password_confirmation: req.password_confirmation,
    };
    let _resp = client.register(req).await?;
    Ok(RegisterResponse {})
}

fn validate_register(req: &RegisterRequest) -> Result<(), AuthError> {
    if req.name.is_empty() {
        return Err(AuthError::InvalidArgument("name is empty".to_string()));
    }
    if req.email.is_empty() {
        return Err(AuthError::InvalidArgument("email is empty".to_string()));
    }
    if req.password.is_empty() {
        return Err(AuthError::InvalidArgument("password is empty".to_string()));
    }
    if req.password_confirmation.is_empty() {
        return Err(AuthError::InvalidArgument(
            "password_confirmation is empty".to_string(),
        ));
    }
    Ok(())
}

pub async fn login(
    request: HttpRequest,
    req: LoginRequest,
    client: &AuthClient,
) -> Result<LoginResponse, AuthError> {
    validate_login(&req)?;

    let req = auth_dto::LoginRequest {
        name: req.name,
        password: req.password,
    };
    let resp = client.login(req).await?;
    if resp.access_token.is_empty() {
        return Err(AuthError::WrongPassword);
    }

    // attach a verified user identity to the active session
    Identity::login(&request.extensions(), resp.user.name.into()).unwrap();

    Ok(LoginResponse {})
}

fn validate_login(req: &LoginRequest) -> Result<(), AuthError> {
    if req.name.is_empty() {
        return Err(AuthError::InvalidArgument("name is empty".to_string()));
    }
    if req.password.is_empty() {
        return Err(AuthError::InvalidArgument("password is empty".to_string()));
    }
    Ok(())
}
