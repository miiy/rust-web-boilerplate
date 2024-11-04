use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::derive::Display;

#[derive(Debug, Display)]
pub enum AppError {
    #[display("Bad Request: {_0}")]
    BadRequest(String),

    #[display("Unauthorized")]
    Unauthorized,

    #[display("Payment Required")]
    PaymentRequired,

    #[display("Forbidden")]
    Forbidden,

    #[display("Not Found")]
    NotFound,

    #[display("Too Many Requests")]
    TooManyRequests,

    #[display("Internal Server Error")]
    InternalServerError,

    #[display("Service Unavailable")]
    ServiceUnavailable,
}

impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::PaymentRequired => StatusCode::PAYMENT_REQUIRED,
            AppError::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::BadRequest(_) => HttpResponse::BadRequest().body(self.to_string()),
            AppError::Unauthorized => HttpResponse::Unauthorized().body(self.to_string()),
            AppError::Forbidden => HttpResponse::Forbidden().body(self.to_string()),
            AppError::NotFound => HttpResponse::NotFound().body(self.to_string()),
            AppError::PaymentRequired => HttpResponse::PaymentRequired().body(self.to_string()),
            AppError::TooManyRequests => HttpResponse::TooManyRequests().body(self.to_string()),
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().body(self.to_string())
            }
            AppError::ServiceUnavailable => HttpResponse::ServiceUnavailable().body(self.to_string()),
        }
    }
}
