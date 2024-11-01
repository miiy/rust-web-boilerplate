use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::derive::Display;

#[derive(Debug, Display)]
pub enum AppError {
    #[display("Internal Server Error")]
    InternalServerError,

    #[display("Bad Request: {_0}")]
    BadRequest(String),

    #[display("Unauthorized")]
    Unauthorized,
}

impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().body(self.to_string())
            }
            AppError::BadRequest(_) => HttpResponse::BadRequest().body(self.to_string()),
            AppError::Unauthorized => HttpResponse::Unauthorized().body(self.to_string()),
        }
    }
}
