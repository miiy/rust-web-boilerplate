use crate::api::error::{APIError, ErrorEntity};
use derive_more::Display;
use redis::RedisError;
use std::error::Error;

#[derive(Debug, Display)]
pub enum UserError {
    #[display("params error: {_0}")]
    Params(String),
    #[display("service error: {_0}")]
    Service(String),
    #[display("database error: {source}")]
    Database { source: sqlx::Error },
    #[display("redis error: {source}")]
    Redis { source: RedisError },
    #[display("user not found")]
    NotFound,
}

impl UserError {
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

impl Error for UserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database { source: ref e } => Some(e),
            Self::Redis { source: ref e } => Some(e),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for UserError {
    fn from(from: sqlx::Error) -> UserError {
        UserError::Database { source: from }
    }
}

impl From<RedisError> for UserError {
    fn from(from: RedisError) -> UserError {
        UserError::Redis { source: from }
    }
}

impl From<UserError> for APIError {
    fn from(from: UserError) -> APIError {
        let e = ErrorEntity {
            code: from.code(),
            message: from.to_string(),
        };
        match from {
            UserError::Params(_) => APIError::BadRequest(e),
            UserError::Service(_) | UserError::Database { .. } | UserError::Redis { .. } => {
                APIError::InternalError(e)
            }
            UserError::NotFound => APIError::NotFound(e),
        }
    }
}
