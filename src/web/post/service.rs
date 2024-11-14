use super::dto::*;
use crate::api::post::service::Service;
use crate::web::error::AppError;
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
        .map(|item| Post {
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

pub async fn detail(req: &DetailRequest, app_state: &AppState) -> Result<DetailResponse, AppError> {
    let resp =
        Service::detail(req.id, &app_state.db)
            .await
            .map_err(|e| AppError::ServiceError {
                source: Box::new(e),
            })?;
    Ok(DetailResponse {
        post: Post {
            id: resp.id,
            category_id: resp.category_id,
            title: resp.title,
            author: resp.author,
            content: resp.content,
            create_time: resp.create_time.date().to_string(),
            update_time: resp.update_time.date().to_string(),
        },
    })
}

pub async fn create(
    _req: &CreateRequest,
    _app_state: &AppState,
) -> Result<CreateResponse, AppError> {
    Ok(CreateResponse {})
}

pub async fn edit(req: &EditRequest, app_state: &AppState) -> Result<EditResponse, AppError> {
    let detail_resp = detail(&DetailRequest { id: req.id }, app_state).await?;
    Ok(EditResponse {
        post: detail_resp.post,
    })
}
