use super::dto;
use super::service;
use super::template::LoginTemplate;
use super::template::RegisterTemplate;
use crate::web::error::AppError;
use crate::AppState;
use actix_session::Session;
use actix_web::{web, Error, HttpResponse};

// GET /register
pub async fn register(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = RegisterTemplate {
        app_name: app_state.app_name.clone(),
        page_title: "Register".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
    };
    let html = app_state
        .tera
        .render(
            "auth/register.html",
            &tera::Context::from_serialize(&template).map_err(|e| AppError::from(e))?,
        )
        .map_err(|e| AppError::from(e))?;
    Ok(HttpResponse::Ok().body(html))
}

// GET /login
pub async fn login(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = LoginTemplate {
        app_name: app_state.app_name.clone(),
        page_title: "Register".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
    };
    let html = app_state
        .tera
        .render(
            "auth/login.html",
            &tera::Context::from_serialize(&template).map_err(|e| AppError::from(e))?,
        )
        .map_err(|e| AppError::from(e))?;
    Ok(HttpResponse::Ok().body(html))
}

// POST /login
pub async fn session_login(
    params: web::Json<dto::LoginRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let _ok = service::login(params.into_inner(), session)
        .await
        .map_err(|e| AppError::from(e))?;
    Ok(HttpResponse::Ok().body(""))
}

pub async fn me(session: Session) -> Result<HttpResponse, Error> {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);
    println!("user_id: {:?}", user_id);
    Ok(HttpResponse::Ok().body(""))
}

// GET /logout
pub async fn logout(session: Session) -> Result<HttpResponse, Error> {
    service::logout(session)
        .await
        .map_err(|e| AppError::from(e))?;
    Ok(HttpResponse::Found()
        .append_header(("location", "/"))
        .finish())
}
