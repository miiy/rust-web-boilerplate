use super::dto::{LoginRequest, LoginResponse};
use super::error::AuthError;
use actix_session::Session;

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
