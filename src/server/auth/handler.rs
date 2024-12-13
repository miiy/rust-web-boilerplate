use super::dto::*;
use super::service;
use super::template::*;
use crate::error::AppError;
use crate::AppState;
use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse};

// GET /register
pub async fn show_register(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = RegisterTemplate {};
    let html =
        app_state
            .template
            .render(REGISTER_TEMPLATE_PATH, REGISTER_RESOURCE_NAME, &template)?;
    Ok(HttpResponse::Ok().body(html))
}

// POST /register
pub async fn register(
    params: web::Json<RegisterRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let client = &app_state.auth_client;
    let _resp = service::register(params.into_inner(), client)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Found()
        .append_header(("location", "/login"))
        .finish())
}

// GET /login
pub async fn show_login(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = LoginTemplate {};
    let html = app_state
        .template
        .render(LOGIN_TEMPLATE_PATH, LOGIN_RESOURCE_NAME, &template)?;
    Ok(HttpResponse::Ok().body(html))
}

// POST /login
pub async fn login(
    request: HttpRequest,
    params: web::Json<LoginRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let client = &app_state.auth_client;
    let _resp = service::login(request, params.into_inner(), client)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Found()
        .append_header(("location", "/"))
        .finish())
}

pub async fn profile(user: Option<Identity>) -> Result<HttpResponse, Error> {
    if let Some(user) = user {
        let resp = format!("Welcome! {}", user.id().unwrap());
        return Ok(HttpResponse::Ok().body(resp));
    }
    Err(AppError::Unauthorized.into())
}

// GET /logout
pub async fn logout(user: Identity) -> Result<HttpResponse, Error> {
    user.logout();
    Ok(HttpResponse::Found()
        .append_header(("location", "/"))
        .finish())
}
