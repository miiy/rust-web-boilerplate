use api::jwt::JWT;
use web::template::Template;
use actix_web::{web::JsonConfig, error, HttpResponse, HttpRequest};
use api::error::{ErrorEntity, ErrorResponse};

pub mod api;
pub mod config;
pub mod db;
pub mod middleware;
pub mod web;

pub struct AppState {
    pub db: sqlx::MySqlPool,
    pub redis: redis::Client,
    pub template: Template,
    pub jwt: JWT,
}

// Actix Web Json Config
pub fn json_config() -> JsonConfig {
    JsonConfig::default()
        .error_handler(json_error_handler)
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
