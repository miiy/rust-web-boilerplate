use super::dto::*;
use super::template::*;
use crate::web::error::AppError;
use crate::AppState;
use actix_web::{web, Error, HttpRequest, HttpResponse};
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

    let req = IndexRequest {
        page: page,
        page_size: page_size,
    };
    let resp = super::service::index(&req, &app_state).await?;

    let template = IndexTemplate { data: resp.data };
    let html = app_state.template.render(INDEX_TEMPLATE_PATH, INDEX_RESOURCE_NAME, &template)?;
    Ok(HttpResponse::Ok().body(html))
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
    let req = DetailRequest { id: id };
    let resp = super::service::detail(&req, &app_state).await?;
    let template = DetailTemplate {
        post: resp.post,
    };
    let html = app_state.template.render(DETAIL_TEMPLATE_PATH, DETAIL_RESOURCE_NAME, &template)?;
    Ok(HttpResponse::Ok().body(html))
}

// GET /posts_create
pub async fn create(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let req = CreateRequest {};
    let _resp = super::service::create(&req, &app_state).await?;

    let template = CreateTemplate {};
    let html = app_state.template.render(CREATE_TEMPLATE_PATH, CREATE_RESOURCE_NAME, &template)?;
    Ok(HttpResponse::Ok().body(html))
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

    let req = EditRequest { id: id };
    let _resp = super::service::edit(&req, &app_state).await;

    let template = EditTemplate {};
    let html = app_state.template.render(EDIT_TEMPLATE_PATH, EDIT_RESOURCE_NAME, &template)?;
    Ok(HttpResponse::Ok().body(html))
}
