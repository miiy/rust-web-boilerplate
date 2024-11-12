use super::dto::{
    CreateRequest, CreateResponse, DetailRequest, DetailResponse, EditRequest, EditResponse,
    IndexRequest, IndexResponse,
};
use crate::api::post::service::Service;
use crate::web::error::AppError;
use crate::web::post::dto;
use crate::AppState;

pub async fn index(req: &IndexRequest, app_state: &AppState) -> Result<IndexResponse, AppError> {
    let resp = Service::lists(req.page, req.page_size, &app_state.db)
        .await
        .map_err(|e| AppError::InternalServerError {
            source: Box::new(e),
        })?;

    let lists = resp
        .lists
        .into_iter()
        .map(|item| dto::Post {
            id: item.id,
            category_id: item.category_id,
            title: item.title,
            author: item.author,
            content: item.content,
            create_time: item.create_time.date().to_string(),
            update_time: item.update_time.date().to_string(),
        })
        .collect();

    Ok(IndexResponse {
        page: req.page,
        lists: lists,
    })
}

pub async fn detail(
    _req: &DetailRequest,
    _app_state: &AppState,
) -> Result<DetailResponse, AppError> {
    Ok(DetailResponse {})
}

pub async fn create(
    _req: &CreateRequest,
    _app_state: &AppState,
) -> Result<CreateResponse, AppError> {
    Ok(CreateResponse {})
}

pub async fn edit(_req: &EditRequest, _app_state: &AppState) -> Result<EditResponse, AppError> {
    Ok(EditResponse {})
}
