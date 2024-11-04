use super::dto::{RegisterRequest, RegisterResponse, LoginRequest, LoginResponse};
use super::model::User;
use super::error::AuthError;
use sqlx::MySqlPool;
use time::OffsetDateTime;
use bcrypt;

pub struct Service;

impl Service {
    pub async fn register(
        req: RegisterRequest,
        pool: &MySqlPool,
    ) -> Result<RegisterResponse, AuthError> {
        if req.password != req.password_confirmation {
            AuthError::Params("password error".to_string());
        }

        let exists = User::email_exists(pool, &req.email).await.map_err(|e| AuthError::from(e))?;
        if exists {
            return Err(AuthError::UserExists)
        }

        let hashed = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST).unwrap();

        let user = User {
            id: 0,
            name: req.name,
            email: req.email,
            password: hashed,
            create_time: Some(OffsetDateTime::now_utc()),
            update_time: Some(OffsetDateTime::now_utc()),
        };
        let user_id = User::create(&pool, &user)
            .await
            .map_err(|e| AuthError::from(e))?;

        let resp = RegisterResponse { id: user_id };
        Ok(resp)
    }

    pub async fn login(
        req: LoginRequest,
        pool: &MySqlPool,
    ) -> Result<LoginResponse, AuthError> {
        let user_potion = User::find_by_name(&pool, req.name)
            .await
            .map_err(|e| AuthError::from(e))?;
        if let Some(user) = user_potion {
            let valid = bcrypt::verify(req.password, &user.password).unwrap();
            if !valid {
                return Err(AuthError::Params("wrong password".to_string()));
            }
            let resp = LoginResponse {
                id: user.id,
                token: "".to_string(),
            };
            Ok(resp)
        } else {
            Err(AuthError::NotFound)
        }
    }
}
