use crate::api::error::{APIError, ErrorEntity};
use derive_more::Display;
use redis::RedisError;
use std::error::Error;

#[derive(Debug, Display)]
pub enum PostError {
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

impl PostError {
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

impl Error for PostError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database { source: ref e } => Some(e),
            Self::Redis { source: ref e } => Some(e),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for PostError {
    fn from(from: sqlx::Error) -> PostError {
        PostError::Database { source: from }
    }
}

impl From<RedisError> for PostError {
    fn from(from: RedisError) -> PostError {
        PostError::Redis { source: from }
    }
}

impl From<PostError> for APIError {
    fn from(from: PostError) -> APIError {
        let e = ErrorEntity {
            code: from.code(),
            message: from.to_string(),
        };
        match from {
            PostError::Params(_) => APIError::BadRequest(e),
            PostError::Service(_) | PostError::Database { .. } | PostError::Redis { .. } => {
                APIError::InternalError(e)
            }
            PostError::NotFound => APIError::NotFound(e),
        }
    }
}
