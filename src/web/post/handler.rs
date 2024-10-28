use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use std::collections::HashMap;
use crate::web::errors::ServiceError;

// /posts
pub async fn index(req: HttpRequest, app_state: web::Data<AppState>) -> Result<HttpResponse, ServiceError> {
    let query = web::Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
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

// /posts/{id}
pub async fn detail(path: web::Path<String>, app_state: web::Data<AppState>) -> Result<HttpResponse, ServiceError> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|_e| ServiceError::BadRequest)?;
    let t = super::service::detail(id, app_state).await;
    Ok(HttpResponse::Ok().body(t.render().unwrap()))
}


pub async fn create(app_state: web::Data<AppState>) -> impl Responder {
    let t = super::service::create(app_state).await;
    HttpResponse::Ok().body(t.render().unwrap())
}


// /posts/{id}/edit
pub async fn edit(path: web::Path<String>, app_state: web::Data<AppState>) -> Result<HttpResponse, ServiceError> {
    let id: u64 = path
        .into_inner()
        .parse()
        .map_err(|_e| ServiceError::BadRequest)?;
    let t = super::service::edit(id, app_state).await;
    Ok(HttpResponse::Ok().body(t.render().unwrap()))
}
