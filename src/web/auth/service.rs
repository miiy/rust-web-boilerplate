use super::dto::LoginRequest;
use super::error::AuthError;
use super::template::{LoginTemplate, RegisterTemplate};
use crate::AppState;
use actix_session::Session;
use actix_web::web;

pub async fn register(app_state: web::Data<AppState>) -> RegisterTemplate {
    let app_name = &app_state.app_name;
    RegisterTemplate {
        app_name: app_name.to_string(),
        page_title: "Register".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
    }
}

pub async fn show_login() -> LoginTemplate {
    LoginTemplate {
        app_name: "".to_string(),
        page_title: "Login".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
    }
}

pub async fn login(req: LoginRequest, session: Session) -> Result<(), AuthError> {
    validate_login(&req)?;
    let id = 1;
    session.insert("user_id", &id)?;
    println!("Logged in user {}", id);
    Ok(())
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
