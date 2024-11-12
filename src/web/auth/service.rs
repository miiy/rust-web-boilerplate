use super::dto::{LoginRequest, RegisterResponse};
use super::error::AuthError;
use crate::web::auth::dto::LoginResponse;
use crate::AppState;
use actix_session::Session;
use actix_web::web;

pub async fn register(_app_state: web::Data<AppState>) -> Result<RegisterResponse, AuthError> {
    Ok(RegisterResponse {})
}

pub async fn login(req: LoginRequest, session: Session) -> Result<LoginResponse, AuthError> {
    validate_login(&req)?;
    let id = 1;
    session.insert("user_id", &id)?;
    println!("Logged in user {}", id);
    Ok(LoginResponse {})
}

fn validate_login(req: &LoginRequest) -> Result<(), AuthError> {
    if req.name.is_empty() || req.password.is_empty() {
        return Err(AuthError::InvalidArgument("".to_string()));
    }
    Ok(())
}

pub async fn logout(session: Session) -> Result<(), AuthError> {
    session.purge();
    Ok(())
}
