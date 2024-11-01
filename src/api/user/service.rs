use super::dto;
use super::model::User;
use crate::api::error::APIError;
use crate::api::user::error::UserError;
use sqlx::MySqlPool;
use time::OffsetDateTime;

pub struct Service;

impl Service {
    pub async fn detail(id: u64, pool: &MySqlPool) -> Result<dto::DetailResponse, UserError> {
        let user_option = User::find(&pool, id)
            .await
            .map_err(|e| UserError::from(e))?;

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
            Err(UserError::NotFound)
        }
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
        };
        let user_id = User::update(&pool, &user)
            .await
            .map_err(|e| UserError::from(e))?;

        Ok(dto::UpdateResponse { id: user_id })
    }
}
