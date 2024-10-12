use actix_web::{error, http::StatusCode, HttpRequest, HttpResponse};
use derive_more::derive::Display;
use serde::Serialize;

#[derive(Debug, Display)]
pub enum APIError {
    #[display("Bad Request")]
    BadRequest(u32),

    #[display("Unauthorized")]
    Unauthorized(u32),

    #[display("Not Found")]
    NotFound(u32),

    #[display("Internal Server Error")]
    InternalError(u32),

    #[display("Gateway Timeout")]
    GatewayTimeout(u32),
}

impl APIError {
    fn error_code(&self) -> u32 {
        match self {
            Self::BadRequest(code) => *code,
            Self::Unauthorized(code) => *code,
            Self::NotFound(code) => *code,
            Self::InternalError(code) => *code,
            Self::GatewayTimeout(code) => *code,
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse<T> {
    code: u32,
    error: T,
}

impl error::ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            code: self.error_code(),
            error: self.to_string(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::GatewayTimeout(_) => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

pub fn json_error_handler(
    err: actix_web::error::JsonPayloadError,
    _req: &HttpRequest,
) -> actix_web::error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(ErrorResponse {
            code: 0,
            error: detail,
        }),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().json(ErrorResponse {
                code: 0,
                error: detail,
            })
        }
        _ => HttpResponse::BadRequest().json(ErrorResponse {
            code: 0,
            error: detail,
        }),
    };
    actix_web::error::InternalError::from_response(err, resp).into()
}
