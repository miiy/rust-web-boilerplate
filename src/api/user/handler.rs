use super::dto;
use super::service::Service;
use crate::api::error::APIError;
use crate::api::user::error::UserError;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};

// GET /users/{id}
pub async fn detail(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|e| APIError::from(UserError::Params(e.to_string())))?;
    let resp = Service::detail(id, &app_state.db)
        .await
        .map_err(|e| APIError::from(e))?;
    Ok(HttpResponse::Ok().json(resp))
}

// PUT /users/{id}
pub async fn update(
    path: web::Path<String>,
    params: web::Json<dto::UpdateRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|e| APIError::from(UserError::Params(e.to_string())))?;
    let resp = Service::update(id, params.into_inner(), &app_state.db)
        .await
        .map_err(|e| APIError::from(e))?;
    Ok(HttpResponse::Ok().json(resp))
}
