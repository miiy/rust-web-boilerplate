use super::service::Service;
use crate::error::APIError;
use actix_web::{get, web, HttpResponse};

#[get("/health")]
async fn health(rds: web::Data<redis::Client>) -> Result<HttpResponse, APIError> {
    Service::health(&rds).await.map_err(|e| {
        log::error!("{e}");
        APIError::InternalError(0)
    })?;

    Ok(HttpResponse::Ok().body(format!("ok!")))
}
