use super::dto;
use super::service;
use crate::web::error::AppError;
use crate::AppState;
use actix_session::Session;
use actix_web::{web, Error, HttpResponse};
use askama::Template;

// GET /register
pub async fn register(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let t = service::register(app_state).await;
    Ok(HttpResponse::Ok().body(t.render().unwrap()))
}

// GET /login
pub async fn login(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let t = service::show_login(app_state).await;
    Ok(HttpResponse::Ok().body(t.render().unwrap()))
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