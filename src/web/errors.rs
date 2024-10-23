use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display("Internal Server Error")]
    InternalServerError,

    #[display("BadRequest: {_0}")]
    BadRequest(String),

    #[display("Unauthorized")]
    Unauthorized,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().body("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(message) => {
                HttpResponse::BadRequest().body(message.to_string())
            }
            ServiceError::Unauthorized => HttpResponse::Unauthorized().body("Unauthorized"),
        }
    }
}
