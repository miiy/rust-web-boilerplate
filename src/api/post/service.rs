use super::dto::*;
use super::error::PostError;
use super::model::Post;
use sqlx::MySqlPool;
use time::OffsetDateTime;

pub struct Service;

impl Service {
    pub async fn lists(
        page: u32,
        page_size: u32,
        pool: &MySqlPool,
    ) -> Result<ListResponse, PostError> {
        let posts_result = Post::find_all(&pool, page, page_size).await?;

        let post_items: Vec<ListResponseItem> = posts_result
            .into_iter()
            .map(|post| ListResponseItem {
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

        Ok(ListResponse { lists: post_items })
    }

    pub async fn detail(id: u64, pool: &MySqlPool) -> Result<DetailResponse, PostError> {
        let post_option = Post::find(&pool, id).await?;

        if let Some(post) = post_option {
            let resp = DetailResponse {
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

    pub async fn create(req: CreateRequest, pool: &MySqlPool) -> Result<CreateResponse, PostError> {
        let post = Post {
            id: 0,
            category_id: req.category_id,
            title: req.title,
            author: req.author,
            content: req.content,
            create_time: Some(OffsetDateTime::now_utc()),
            update_time: Some(OffsetDateTime::now_utc()),
        };
        let post_id = Post::create(&pool, &post).await?;

        let resp = CreateResponse { id: post_id };
        Ok(resp)
    }

    pub async fn update(
        id: u64,
        req: UpdateRequest,
        pool: &MySqlPool,
    ) -> Result<UpdateResponse, PostError> {
        let post = Post {
            id: id,
            category_id: req.category_id,
            title: req.title,
            author: req.author,
            content: req.content,
            create_time: None,
            update_time: Some(OffsetDateTime::now_utc()),
        };
        let post_id = Post::update(&pool, &post).await?;

        Ok(UpdateResponse { id: post_id })
    }

    pub async fn delete(id: u64, pool: &MySqlPool) -> Result<DeleteResponse, PostError> {
        let _rf = Post::soft_delete(&pool, id).await?;
        Ok(DeleteResponse {})
    }
}
