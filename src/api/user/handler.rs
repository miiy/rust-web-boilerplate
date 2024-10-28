use super::dto;
use super::service::Service;
use crate::api::errors::APIError;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::collections::HashMap;

#[get("/users")]
async fn list(req: HttpRequest, app_state: web::Data<AppState>) -> Result<HttpResponse, APIError> {
    let query = web::Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
    let page = query
        .get("page")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(1);
    let page_size = query
        .get("page_size")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(20);
    let resp = Service::lists(page, page_size, &app_state.db).await?;

    Ok(HttpResponse::Ok().json(resp))
}

#[get("/users/{id}")]
async fn detail(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, APIError> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|_e| APIError::BadRequest(0))?;
    let resp = Service::detail(id, &app_state.db).await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[post("/users")]
async fn create(
    params: web::Json<dto::CreateRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, APIError> {
    let resp = Service::create(params.into_inner(), &app_state.db).await?;
    Ok(HttpResponse::Created().json(resp))
}

#[put("/users/{id}")]
async fn update(
    path: web::Path<String>,
    params: web::Json<dto::UpdateRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, APIError> {
    let id: u64 = path
        .into_inner()
        .parse()
        .map_err(|_e| APIError::BadRequest(0))?;
    let resp = Service::update(id, params.into_inner(), &app_state.db).await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[delete("/users/{id}")]
async fn delete(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, APIError> {
    let id: u64 = path
        .into_inner()
        .parse()
        .map_err(|_e| APIError::BadRequest(0))?;
    let resp = Service::delete(id, &app_state.db).await?;
    Ok(HttpResponse::Ok().json(resp))
}
