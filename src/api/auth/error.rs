use crate::api::error::{APIError, ErrorEntity};
use bcrypt::BcryptError;
use derive_more::Display;
use redis::RedisError;
use std::error::Error;

#[derive(Debug, Display)]
pub enum AuthError {
    #[display("invalid argument: {_0}")]
    InvalidArgument(String),
    #[display("service error: {_0}")]
    Service(String),
    #[display("database error: {source}")]
    Database { source: sqlx::Error },
    #[display("redis error: {source}")]
    Redis { source: RedisError },
    #[display("bcrypt error: {source}")]
    Bcrypt { source: BcryptError },
    #[display("jwt error: {source}")]
    JWT { source: jsonwebtoken::errors::Error },
    #[display("user not found")]
    UserNotFound,
    // register
    #[display("username exists")]
    UsernameAlreadyExists,
    #[display("email exists")]
    EmailAlreadyExists,
    #[display("password differ")]
    PasswordDiffer,
    // login
    #[display("wrong password")]
    WrongPassword,
}

impl AuthError {
    pub fn code(&self) -> i32 {
        match self {
            Self::InvalidArgument(_) => 10001,
            Self::WrongPassword => 10002,
            Self::Service(_) => 10003,
            Self::Database { .. } => 10004,
            Self::Redis { .. } => 10005,
            Self::Bcrypt { .. } => 10006,
            Self::JWT { .. } => 10007,
            Self::UserNotFound => 10008,
            Self::EmailAlreadyExists => 10009,
            Self::UsernameAlreadyExists => 10010,
            Self::PasswordDiffer => 10011,
        }
    }
}

impl Error for AuthError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database { source: ref e } => Some(e),
            Self::Redis { source: ref e } => Some(e),
            Self::Bcrypt { source: ref e } => Some(e),
            Self::JWT { source: ref e } => Some(e),
            _ => None,
        }
    }
}

impl From<AuthError> for APIError {
    fn from(from: AuthError) -> Self {
        let e = ErrorEntity {
            code: from.code(),
            message: from.to_string(),
        };
        match from {
            AuthError::InvalidArgument(_)
            | AuthError::PasswordDiffer
            | AuthError::WrongPassword => APIError::BadRequest(e),
            AuthError::UsernameAlreadyExists | AuthError::EmailAlreadyExists => {
                APIError::CONFLICT(e)
            }
            AuthError::Service(_)
            | AuthError::Database { .. }
            | AuthError::Redis { .. }
            | AuthError::Bcrypt { .. }
            | AuthError::JWT { .. } => APIError::InternalError(e),
            AuthError::UserNotFound => APIError::NotFound(e),
        }
    }
}

impl From<sqlx::Error> for AuthError {
    fn from(from: sqlx::Error) -> AuthError {
        AuthError::Database { source: from }
    }
}

impl From<RedisError> for AuthError {
    fn from(from: RedisError) -> AuthError {
        AuthError::Redis { source: from }
    }
}

impl From<BcryptError> for AuthError {
    fn from(from: BcryptError) -> AuthError {
        AuthError::Bcrypt { source: from }
    }
}

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(from: jsonwebtoken::errors::Error) -> AuthError {
        AuthError::Service(from.to_string())
    }
}
