use super::dto;
use super::model::User;
use crate::api::user::error::UserError;
use sqlx::MySqlPool;
use time::OffsetDateTime;

pub struct Service;

impl Service {
    pub async fn detail(name: &str, pool: &MySqlPool) -> Result<dto::MeResponse, UserError> {
        let user_option = User::find_by_name(&pool, name).await?;

        if let Some(user) = user_option {
            let resp = dto::MeResponse {
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
}
