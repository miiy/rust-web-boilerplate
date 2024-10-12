use super::dto;
use super::service::Service;
use crate::error::APIError;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use sqlx::MySqlPool;
use std::collections::HashMap;

#[get("/users")]
async fn list(req: HttpRequest, pool: web::Data<MySqlPool>) -> Result<HttpResponse, APIError> {
    let query = web::Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
    let page = query
        .get("page")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(1);
    let page_size = query
        .get("page_size")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(20);
    let resp = Service::lists(page, page_size, &pool).await?;

    Ok(HttpResponse::Ok().json(resp))
}

#[get("/users/{id}")]
async fn detail(
    path: web::Path<String>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, APIError> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|_e| APIError::BadRequest(0))?;
    let resp = Service::detail(id, &pool).await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[post("/users")]
async fn create(
    params: web::Json<dto::CreateRequest>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, APIError> {
    let resp = Service::create(params.into_inner(), &pool).await?;
    Ok(HttpResponse::Created().json(resp))
}

#[put("/users/{id}")]
async fn update(
    path: web::Path<String>,
    params: web::Json<dto::UpdateRequest>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, APIError> {
    let id: u64 = path
        .into_inner()
        .parse()
        .map_err(|_e| APIError::BadRequest(0))?;
    let resp = Service::update(id, params.into_inner(), &pool).await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[delete("/users/{id}")]
async fn delete(
    path: web::Path<String>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, APIError> {
    let id: u64 = path
        .into_inner()
        .parse()
        .map_err(|_e| APIError::BadRequest(0))?;
    let resp = Service::delete(id, &pool).await?;
    Ok(HttpResponse::Ok().json(resp))
}
