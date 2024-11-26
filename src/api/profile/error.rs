use crate::api::error::{APIError, ErrorEntity};
use derive_more::Display;
use redis::RedisError;
use std::error::Error;

#[derive(Debug, Display)]
pub enum ProfileError {
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

impl ProfileError {
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

impl Error for ProfileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database { source: ref e } => Some(e),
            Self::Redis { source: ref e } => Some(e),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for ProfileError {
    fn from(from: sqlx::Error) -> ProfileError {
        ProfileError::Database { source: from }
    }
}

impl From<RedisError> for ProfileError {
    fn from(from: RedisError) -> ProfileError {
        ProfileError::Redis { source: from }
    }
}

impl From<ProfileError> for APIError {
    fn from(from: ProfileError) -> APIError {
        let e = ErrorEntity {
            code: from.code(),
            message: from.to_string(),
        };
        match from {
            ProfileError::Params(_) => APIError::BadRequest(e),
            ProfileError::Service(_) | ProfileError::Database { .. } | ProfileError::Redis { .. } => {
                APIError::InternalError(e)
            }
            ProfileError::NotFound => APIError::NotFound(e),
        }
    }
}
