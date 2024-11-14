use super::dto::*;
use super::template::*;
use crate::web::error::AppError;
use crate::web::template;
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

    let template = IndexTemplate {
        page_title: "Home".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
        lists: resp.lists,
    };
    let html = template::render("post/index.html", &template, &app_state)?;
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
    let _resp = super::service::detail(&req, &app_state).await?;
    let template = DetailTemplate {
        page_title: "Home".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
    };
    let html = template::render("post/detail.html", &template, &app_state)?;
    Ok(HttpResponse::Ok().body(html))
}

// GET /posts_create
pub async fn create(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let req = CreateRequest {};
    let _resp = super::service::create(&req, &app_state).await?;

    let template = CreateTemplate {
        page_title: "create".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
    };
    let html = template::render("post/create.html", &template, &app_state)?;
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

    let template = EditTemplate {
        page_title: "edit".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
    };
    let html = template::render("post/edit.html", &template, &app_state)?;
    Ok(HttpResponse::Ok().body(html))
}
