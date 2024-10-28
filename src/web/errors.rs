use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::derive::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display("Internal Server Error")]
    InternalServerError,

    #[display("BadRequest")]
    BadRequest,

    #[display("Unauthorized")]
    Unauthorized,
}

impl error::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self::ServiceError::InternalServerError {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().body("Internal Server Error, Please try later")
            }
            // ServiceError::BadRequest(message) => {
            //     HttpResponse::BadRequest().body(message.to_string())
            // }
            ServiceError::BadRequest => HttpResponse::BadRequest().body("Bad request"),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().body("Unauthorized"),
        }
    }

}
