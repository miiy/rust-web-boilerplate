use super::dto;
use super::model::User;
use crate::api::auth::dto::LoginResponse;
use crate::api::auth::error::AuthError;
use sqlx::MySqlPool;
use time::OffsetDateTime;

pub struct Service;

impl Service {
    pub async fn register(
        req: dto::RegisterRequest,
        pool: &MySqlPool,
    ) -> Result<dto::RegisterResponse, AuthError> {
        let user = User {
            id: 0,
            name: req.name,
            email: req.email,
            create_time: Some(OffsetDateTime::now_utc()),
            update_time: Some(OffsetDateTime::now_utc()),
        };
        let user_id = User::create(&pool, &user)
            .await
            .map_err(|e| AuthError::from(e))?;

        let resp = dto::RegisterResponse { id: user_id };
        Ok(resp)
    }

    pub async fn login(
        req: dto::LoginRequest,
        pool: &MySqlPool,
    ) -> Result<LoginResponse, AuthError> {
        let user_potion = User::find(&pool, req.name, req.password)
            .await
            .map_err(|e| AuthError::from(e))?;
        if let Some(user) = user_potion {
            let resp = dto::LoginResponse {
                id: user.id,
                token: "".to_string(),
            };
            Ok(resp)
        } else {
            Err(AuthError::NotFound)
        }
    }
}
