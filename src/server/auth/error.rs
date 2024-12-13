use crate::error::AppError;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum AuthError {
    #[display("invalid argument: {_0}")]
    InvalidArgument(String),
    #[display("wrong password")]
    WrongPassword,
    #[display("unauthorized")]
    Unauthorized,
    SessionInsertError {
        source: actix_session::SessionInsertError,
    },
}

impl From<AuthError> for AppError {
    fn from(from: AuthError) -> Self {
        match from {
            AuthError::InvalidArgument(msg) => AppError::BadRequest(msg),
            AuthError::WrongPassword => AppError::BadRequest("wrong password".to_string()),
            AuthError::Unauthorized => AppError::Unauthorized,
            AuthError::SessionInsertError { source } => AppError::InternalServerError {
                source: Box::new(source),
            },
        }
    }
}

impl From<actix_session::SessionInsertError> for AuthError {
    fn from(from: actix_session::SessionInsertError) -> Self {
        AuthError::SessionInsertError { source: from }
    }
}

impl From<reqwest::Error> for AuthError {
    fn from(from: reqwest::Error) -> Self {
        AuthError::InvalidArgument(from.to_string())
    }
}
