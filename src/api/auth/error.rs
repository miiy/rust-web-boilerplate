use crate::api::error::{APIError, ErrorEntity};
use derive_more::Display;
use redis::RedisError;
use std::error::Error;

#[derive(Debug, Display)]
pub enum AuthError {
    #[display("params error: {_0}")]
    Params(String),
    #[display("service error: {_0}")]
    Service(String),
    #[display("database error: {source}")]
    Database { source: sqlx::Error },
    #[display("redis error: {source}")]
    Redis { source: RedisError },
    #[display("post not found")]
    NotFound,
}

impl AuthError {
    pub fn code(&self) -> i32 {
        match self {
            Self::Params(_) => 10001,
            Self::Service(_) => 10002,
            Self::Database { .. } => 10003,
            Self::Redis { .. } => 10004,
            Self::NotFound => 10005,
        }
    }
}

impl Error for AuthError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database { source: ref e } => Some(e),
            Self::Redis { source: ref e } => Some(e),
            _ => None,
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

impl From<AuthError> for APIError {
    fn from(from: AuthError) -> APIError {
        let e = ErrorEntity {
            code: from.code(),
            message: from.to_string(),
        };
        match from {
            AuthError::Params(_) => APIError::BadRequest(e),
            AuthError::Service(_) | AuthError::Database { .. } | AuthError::Redis { .. } => {
                APIError::InternalError(e)
            }
            AuthError::NotFound => APIError::NotFound(e),
        }
    }
}
