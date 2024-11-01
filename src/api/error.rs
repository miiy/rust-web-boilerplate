use actix_web::{error, http::StatusCode, HttpRequest, HttpResponse};
use derive_more::derive::Display;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Display)]
pub enum APIError {
    #[display("Bad Request")]
    BadRequest(ErrorEntity),

    #[display("Unauthorized")]
    Unauthorized(ErrorEntity),

    #[display("Not Found")]
    NotFound(ErrorEntity),

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
struct ErrorResponse {
    error: ErrorEntity,
}

impl error::ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
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
            Self::NotFound(e) => {
                log::error!("Not Found: {}", e.to_string());
                HttpResponse::NotFound().json(ErrorResponse { error: e.clone() })
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

pub fn json_error_handler(
    err: error::JsonPayloadError,
    _req: &HttpRequest,
) -> actix_web::error::Error {
    use actix_web::error::JsonPayloadError;
    let error_response = ErrorResponse {
        error: ErrorEntity {
            code: 400,
            message: err.to_string(),
        },
    };
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(error_response),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().json(error_response)
        }
        _ => HttpResponse::BadRequest().json(error_response),
    };
    error::InternalError::from_response(err, resp).into()
}
