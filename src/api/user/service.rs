use super::dto;
use super::model::User;
use crate::api::errors::APIError;
use log;
use sqlx::MySqlPool;
use time::OffsetDateTime;

pub struct Service;

impl Service {
    pub async fn lists(
        page: u32,
        page_size: u32,
        pool: &MySqlPool,
    ) -> Result<dto::ListResponse, APIError> {
        let users_result = User::find_all(&pool, page, page_size).await.map_err(|e| {
            log::error!("{e}");
            APIError::InternalError(0)
        })?;

        let user_items: Vec<dto::ListResponseItem> = users_result
            .into_iter()
            .map(|user| dto::ListResponseItem {
                name: user.name,
                email: user.email,
            })
            .collect();

        Ok(dto::ListResponse { lists: user_items })
    }

    pub async fn detail(id: u64, pool: &MySqlPool) -> Result<dto::DetailResponse, APIError> {
        let user_option = User::find(&pool, id).await.map_err(|e| {
            log::error!("{e}");
            APIError::InternalError(0)
        })?;

        if let Some(user) = user_option {
            let resp = dto::DetailResponse {
                name: user.name,
                email: user.email,
                create_time: user
                    .create_time
                    .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
                update_time: user
                    .update_time
                    .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
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
        let user = User {
            id: 0,
            name: req.name,
            email: req.email,
            create_time: Some(OffsetDateTime::now_utc()),
            update_time: Some(OffsetDateTime::now_utc()),
            delete_time: None,
        };
        let user_id = User::create(&pool, &user).await.map_err(|e| {
            log::error!("{e}");
            APIError::InternalError(0)
        })?;

        let resp = dto::CreateResponse { id: user_id };
        Ok(resp)
    }

    pub async fn update(
        id: u64,
        req: dto::UpdateRequest,
        pool: &MySqlPool,
    ) -> Result<dto::UpdateResponse, APIError> {
        let user = User {
            id: id,
            name: req.name,
            email: req.email,
            create_time: None,
            update_time: Some(OffsetDateTime::now_utc()),
            delete_time: None,
        };
        let user_id = User::update(&pool, &user).await.map_err(|e| {
            log::error!("{e}");
            APIError::InternalError(0)
        })?;

        Ok(dto::UpdateResponse { id: user_id })
    }

    pub async fn delete(id: u64, pool: &MySqlPool) -> Result<dto::DeleteResponse, APIError> {
        let _rf = User::soft_delete(&pool, id).await.map_err(|e| {
            log::error!("{e}");
            APIError::InternalError(0)
        })?;
        Ok(dto::DeleteResponse {})
    }
}
