use super::dto;
use super::model::User;
use super::error::ProfileError;
use sqlx::MySqlPool;
use time::OffsetDateTime;

pub struct Service;

impl Service {
    pub async fn profile(name: &str, pool: &MySqlPool) -> Result<dto::ProfileResponse, ProfileError> {
        let user_option = User::find_by_name(&pool, name).await?;

        if let Some(user) = user_option {
            let resp = dto::ProfileResponse {
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
            Err(ProfileError::NotFound)
        }
    }
}
