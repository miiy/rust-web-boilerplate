use super::dto::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use super::error::AuthError;
use super::model::User;
use super::password;
use sqlx::MySqlPool;
use time::OffsetDateTime;

pub struct Service;

impl Service {
    pub async fn register(
        req: RegisterRequest,
        pool: &MySqlPool,
    ) -> Result<RegisterResponse, AuthError> {
        Self::validate_register(&req)?;

        if !User::check_name_available(pool, &req.name).await? {
            return Err(AuthError::UsernameAlreadyExists);
        }

        if !User::check_email_available(pool, &req.email).await? {
            return Err(AuthError::EmailAlreadyExists);
        }

        let hashed = password::bcrypt_hash(&req.password)?;
        let user = User {
            id: 0,
            name: req.name,
            email: req.email,
            password: hashed,
            create_time: Some(OffsetDateTime::now_utc()),
            update_time: Some(OffsetDateTime::now_utc()),
        };
        let user_id = User::create(&pool, &user).await?;

        let resp = RegisterResponse { id: user_id };
        Ok(resp)
    }

    fn validate_register(req: &RegisterRequest) -> Result<(), AuthError> {
        if req.name.is_empty()
            || req.email.is_empty()
            || req.password.is_empty()
            || req.password_confirmation.is_empty()
        {
            return Err(AuthError::InvalidArgument("".to_string()));
        }
        if !req.password.eq(&req.password_confirmation) {
            return Err(AuthError::PasswordDiffer);
        }
        Ok(())
    }

    pub async fn login(req: LoginRequest, pool: &MySqlPool) -> Result<LoginResponse, AuthError> {
        Self::validate_login(&req)?;

        let user_potion = User::find_by_name(&pool, req.name).await?;
        if let Some(user) = user_potion {
            if !password::bcrypt_verify(&req.password, &user.password)? {
                return Err(AuthError::WrongPassword);
            }
            return Ok(LoginResponse { id: user.id });
        }
        Err(AuthError::WrongPassword)
    }

    fn validate_login(req: &LoginRequest) -> Result<(), AuthError> {
        if req.name.is_empty() || req.password.is_empty() {
            return Err(AuthError::InvalidArgument("".to_string()));
        }
        Ok(())
    }
}
