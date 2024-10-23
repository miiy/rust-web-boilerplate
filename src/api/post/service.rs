use super::dto;
use super::model::Post;
use crate::api::error::APIError;
use log;
use sqlx::MySqlPool;

pub struct Service;

impl Service {
    pub async fn lists(
        page: u32,
        page_size: u32,
        pool: &MySqlPool,
    ) -> Result<dto::ListResponse, APIError> {
        let posts_result = Post::find_all(&pool, page, page_size).await.map_err(|e| {
            log::error!("{e}");
            APIError::InternalError(0)
        })?;

        let post_items: Vec<dto::ListResponseItem> = posts_result
            .into_iter()
            .map(|post| dto::ListResponseItem {
                id: post.id,
                user_id: post.user_id,
                title: post.title,
                content: post.content,
            })
            .collect();

        Ok(dto::ListResponse { lists: post_items })
    }

    pub async fn detail(id: u64, pool: &MySqlPool) -> Result<dto::DetailResponse, APIError> {
        let post_option = Post::find(&pool, id).await.map_err(|e| {
            log::error!("{e}");
            APIError::InternalError(0)
        })?;

        if let Some(post) = post_option {
            let resp = dto::DetailResponse {
                id: post.id,
                user_id: post.user_id,
                title: post.title,
                content: post.content,
            };
            Ok(resp)
        } else {
            Err(APIError::NotFound(0))
        }
    }

    pub async fn create(
        req: dto::CreateRequest,
        pool: &MySqlPool,
    ) -> Result<dto::CreateResponse, APIError> {
        let post = Post {
            id: 0,
            user_id: 0,
            title: req.title.clone(),
            content: req.content.clone(),
        };
        let post_id = Post::create(&pool, &post).await.map_err(|e| {
            log::error!("{e}");
            APIError::InternalError(0)
        })?;

        let resp = dto::CreateResponse { id: post_id };
        Ok(resp)
    }

    pub async fn update(
        id: u64,
        req: dto::UpdateRequest,
        pool: &MySqlPool,
    ) -> Result<dto::UpdateResponse, APIError> {
        let post = Post {
            id: id,
            user_id: 0,
            title: req.title.clone(),
            content: req.content.clone(),
        };
        let post_id = Post::update(&pool, &post).await.map_err(|e| {
            log::error!("{e}");
            APIError::InternalError(0)
        })?;

        Ok(dto::UpdateResponse { id: post_id })
    }

    pub async fn delete(id: u64, pool: &MySqlPool) -> Result<dto::DeleteResponse, APIError> {
        let _rf = Post::delete(&pool, id).await.map_err(|e| {
            log::error!("{e}");
            APIError::InternalError(0)
        })?;
        Ok(dto::DeleteResponse)
        // .json(json!({"message": "Deleted"}))
    }
}
