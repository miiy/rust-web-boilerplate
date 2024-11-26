use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::derive::Display;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Display)]
pub enum APIError {
    #[display("Bad Request")]
    BadRequest(ErrorEntity),

    #[display("Unauthorized")]
    Unauthorized(ErrorEntity),

    #[display("Payment Required")]
    PaymentRequired(ErrorEntity),

    #[display("Forbidden")]
    Forbidden(ErrorEntity),

    #[display("Not Found")]
    NotFound(ErrorEntity),

    #[display("Conflict")]
    CONFLICT(ErrorEntity),

    #[display("Too Many Requests")]
    TooManyRequests(ErrorEntity),

    #[display("Internal Server Error")]
    InternalError(ErrorEntity),

    #[display("Gateway Timeout")]
    GatewayTimeout(ErrorEntity),
}

#[derive(Debug, Serialize, Clone, Display)]
#[display("code: {code}, message: {message}")]
pub struct ErrorEntity {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorEntity,
}

impl error::ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::PaymentRequired(_) => StatusCode::PAYMENT_REQUIRED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::CONFLICT(_) => StatusCode::CONFLICT,
            Self::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::GatewayTimeout(_) => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Self::BadRequest(e) => {
                log::error!("Bad Request: {}", e.to_string());
                HttpResponse::BadRequest().json(ErrorResponse { error: e.clone() })
            }
            Self::Unauthorized(e) => {
                log::error!("Unauthorized: {}", e.to_string());
                HttpResponse::Unauthorized().json(ErrorResponse { error: e.clone() })
            }
            Self::PaymentRequired(e) => {
                log::error!("Payment Required: {}", e.to_string());
                HttpResponse::PaymentRequired().json(ErrorResponse { error: e.clone() })
            }
            Self::Forbidden(e) => {
                log::error!("Forbidden: {}", e.to_string());
                HttpResponse::Forbidden().json(ErrorResponse { error: e.clone() })
            }
            Self::NotFound(e) => {
                log::error!("Not Found: {}", e.to_string());
                HttpResponse::NotFound().json(ErrorResponse { error: e.clone() })
            }
            Self::CONFLICT(e) => {
                log::error!("Conflict: {}", e.to_string());
                HttpResponse::Conflict().json(ErrorResponse { error: e.clone() })
            }
            Self::TooManyRequests(e) => {
                log::error!("Too Many Requests: {}", e.to_string());
                HttpResponse::TooManyRequests().json(ErrorResponse { error: e.clone() })
            }
            Self::InternalError(e) => {
                log::error!("Internal Server Error: {}", e.to_string());
                HttpResponse::InternalServerError().json(ErrorResponse { error: e.clone() })
            }
            Self::GatewayTimeout(e) => {
                log::error!("Gateway Timeout: {}", e.to_string());
                HttpResponse::GatewayTimeout().json(ErrorResponse { error: e.clone() })
            }
        }
    }
}
