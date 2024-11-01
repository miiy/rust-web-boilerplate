use crate::web::error::AppError;
use crate::AppState;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use askama::Template;
use std::collections::HashMap;

// GET /posts
pub async fn index(
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let query = web::Query::<HashMap<String, String>>::from_query(req.query_string())?;
    let page = query
        .get("page")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(1);
    let page_size = query
        .get("page_size")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(20);
    let t = super::service::index(page, page_size, app_state).await?;
    Ok(HttpResponse::Ok().body(t.render().unwrap()))
}

// GET /posts/{id}
pub async fn detail(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let t = super::service::detail(id, app_state).await;
    Ok(HttpResponse::Ok().body(t.render().unwrap()))
}

// GET /posts_create
pub async fn create(app_state: web::Data<AppState>) -> impl Responder {
    let t = super::service::create(app_state).await;
    HttpResponse::Ok().body(t.render().unwrap())
}

// GET /posts/{id}/edit
pub async fn edit(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let t = super::service::edit(id, app_state).await;
    Ok(HttpResponse::Ok().body(t.render().unwrap()))
}
