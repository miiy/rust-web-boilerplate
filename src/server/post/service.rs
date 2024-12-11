use super::dto::*;
use crate::client::post::client::PostClient;
use crate::error::AppError;

pub async fn index(
    req: &IndexRequest,
    post_client: &PostClient,
) -> Result<IndexResponse, AppError> {
    let resp = post_client
        .lists(req.page, req.page_size)
        .await
        .map_err(|e| AppError::InternalServerError {
            source: Box::new(e),
        })?;

    let lists = resp
        .data
        .into_iter()
        .map(|item| Post {
            id: item.id,
            category_id: item.category_id,
            title: item.title,
            author: item.author,
            content: item.content,
            created_at: item.created_at,
            updated_at: item.updated_at,
        })
        .collect();

    Ok(IndexResponse {
        total_pages: resp.total_pages,
        data: lists,
    })
}

pub async fn detail(
    req: &DetailRequest,
    post_client: &PostClient,
) -> Result<DetailResponse, AppError> {
    let resp = post_client
        .detail(req.id)
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
            created_at: resp.created_at,
            updated_at: resp.updated_at,
        },
    })
}

pub async fn create(
    _req: &CreateRequest,
    _post_client: &PostClient,
) -> Result<CreateResponse, AppError> {
    Ok(CreateResponse {})
}

pub async fn edit(req: &EditRequest, post_client: &PostClient) -> Result<EditResponse, AppError> {
    let detail_resp = detail(&DetailRequest { id: req.id }, post_client).await?;
    Ok(EditResponse {
        post: detail_resp.post,
    })
}
