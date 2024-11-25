use super::dto;
use super::service::Service;
use crate::api::error::APIError;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};

// POST /auth/register
pub async fn register(
    params: web::Json<dto::RegisterRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let resp = Service::register(params.into_inner(), &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Created().json(resp))
}

// POST /auth/login
pub async fn login(
    params: web::Json<dto::LoginRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let resp = Service::login(params.into_inner(), &app_state.db, &app_state.jwt)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}
