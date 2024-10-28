use crate::api::post::service;
use super::template::{IndexTemplate, DetailTemplate, CreateTemplate, EditTemplate};
use actix_web::web;
use crate::AppState;
use crate::web::post::dto;
use crate::web::errors::ServiceError;

pub async fn index(page: u32, page_size: u32, app_state: web::Data<AppState>) -> Result<IndexTemplate, ServiceError> {
    let app_name = &app_state.app_name;
    let resp = service::Service::lists(page, page_size, &app_state.db).await.map_err(|e| {
        log::error!("{e}");
        ServiceError::InternalServerError
    })?;

    let lists = resp.lists.into_iter().map(|item| {
        dto::Post {
            id: item.id,
            category_id: item.category_id,
            title: item.title,
            author: item.author,
            content: item.content,
            create_time: item.create_time.date().to_string(),
            update_time: item.update_time.date().to_string()
        }
    }).collect();

    Ok(IndexTemplate {
        app_name: app_name.to_string(),
        page_title: "Home".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
        lists: lists,
    })
}

pub async fn detail(_id: u64, app_state: web::Data<AppState>) -> DetailTemplate {
    let app_name = &app_state.app_name;
    DetailTemplate {
        app_name: app_name.to_string(),
        page_title: "Home".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),

    }
}

pub async fn create(app_state: web::Data<AppState>) -> CreateTemplate {
    let app_name = &app_state.app_name;
    CreateTemplate {
        app_name: app_name.to_string(),
        page_title: "Home".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
    }
}

pub async fn edit(_id: u64, app_state: web::Data<AppState>) -> EditTemplate {
    let app_name = &app_state.app_name;
    EditTemplate {
        app_name: app_name.to_string(),
        page_title: "Home".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
    }
}
