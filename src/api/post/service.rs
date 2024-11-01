use super::model::Post;
use super::{dto, error};
use error::PostError;
use sqlx::MySqlPool;
use time::OffsetDateTime;

pub struct Service;

impl Service {
    pub async fn lists(
        page: u32,
        page_size: u32,
        pool: &MySqlPool,
    ) -> Result<dto::ListResponse, PostError> {
        let posts_result = Post::find_all(&pool, page, page_size)
            .await
            .map_err(|e| PostError::from(e))?;

        let post_items: Vec<dto::ListResponseItem> = posts_result
            .into_iter()
            .map(|post| dto::ListResponseItem {
                id: post.id,
                category_id: post.category_id,
                title: post.title,
                author: post.author,
                content: post.content,
                create_time: post
                    .create_time
                    .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
                update_time: post
                    .update_time
                    .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
            })
            .collect();

        Ok(dto::ListResponse { lists: post_items })
    }

    pub async fn detail(id: u64, pool: &MySqlPool) -> Result<dto::DetailResponse, PostError> {
        let post_option = Post::find(&pool, id)
            .await
            .map_err(|e| PostError::from(e))?;

        if let Some(post) = post_option {
            let resp = dto::DetailResponse {
                id: post.id,
                category_id: post.category_id,
                title: post.title,
                author: post.author,
                content: post.content,
                create_time: post
                    .create_time
                    .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
                update_time: post
                    .update_time
                    .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
            };
            Ok(resp)
        } else {
            Err(PostError::NotFound)
        }
    }

    pub async fn create(
        req: dto::CreateRequest,
        pool: &MySqlPool,
    ) -> Result<dto::CreateResponse, PostError> {
        let post = Post {
            id: 0,
            category_id: req.category_id,
            title: req.title,
            author: req.author,
            content: req.content,
            create_time: Some(OffsetDateTime::now_utc()),
            update_time: Some(OffsetDateTime::now_utc()),
        };
        let post_id = Post::create(&pool, &post)
            .await
            .map_err(|e| PostError::from(e))?;

        let resp = dto::CreateResponse { id: post_id };
        Ok(resp)
    }

    pub async fn update(
        id: u64,
        req: dto::UpdateRequest,
        pool: &MySqlPool,
    ) -> Result<dto::UpdateResponse, PostError> {
        let post = Post {
            id: id,
            category_id: req.category_id,
            title: req.title,
            author: req.author,
            content: req.content,
            create_time: None,
            update_time: Some(OffsetDateTime::now_utc()),
        };
        let post_id = Post::update(&pool, &post)
            .await
            .map_err(|e| PostError::from(e))?;

        Ok(dto::UpdateResponse { id: post_id })
    }

    pub async fn delete(id: u64, pool: &MySqlPool) -> Result<dto::DeleteResponse, PostError> {
        let _rf = Post::soft_delete(&pool, id)
            .await
            .map_err(|e| PostError::from(e))?;
        Ok(dto::DeleteResponse {})
    }
}
