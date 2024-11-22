use super::dto::*;
use super::template::*;
use crate::web::error::AppError;
use crate::web::pagination;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IndexQuery {
    pub page: Option<String>,
}

// GET /posts
pub async fn index(
    path: Option<web::Path<u32>>,
    query: web::Query<IndexQuery>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let page = path
        .map(|p| p.into_inner())
        .or_else(|| query.page.as_ref().and_then(|p| p.parse::<u32>().ok()))
        .unwrap_or(1);
    let page_size = 10;

    let req = IndexRequest {
        page: page,
        page_size: page_size,
    };
    println!("page: {}", page);
    let resp = super::service::index(&req, &app_state).await?;

    let page_links =
        pagination::build_links("/posts/pages", page as usize, resp.total_pages as usize);

    let template = IndexTemplate {
        data: resp.data,
        pagination: page_links,
    };
    let html = app_state
        .template
        .render(INDEX_TEMPLATE_PATH, INDEX_RESOURCE_NAME, &template)?;
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
    let template = DetailTemplate { post: resp.post };
    let html = app_state
        .template
        .render(DETAIL_TEMPLATE_PATH, DETAIL_RESOURCE_NAME, &template)?;
    Ok(HttpResponse::Ok().body(html))
}

// GET /posts_create
pub async fn create(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let req = CreateRequest {};
    let _resp = super::service::create(&req, &app_state).await?;

    let template = CreateTemplate {};
    let html = app_state
        .template
        .render(CREATE_TEMPLATE_PATH, CREATE_RESOURCE_NAME, &template)?;
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
    let html = app_state
        .template
        .render(EDIT_TEMPLATE_PATH, EDIT_RESOURCE_NAME, &template)?;
    Ok(HttpResponse::Ok().body(html))
}
