use crate::web::error::AppError;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum AuthError {
    #[display("invalid argument: {_0}")]
    InvalidArgument(String),
    SessionInsertError {
        source: actix_session::SessionInsertError,
    },
}

impl From<AuthError> for AppError {
    fn from(from: AuthError) -> Self {
        match from {
            AuthError::InvalidArgument(msg) => AppError::BadRequest(msg),
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
