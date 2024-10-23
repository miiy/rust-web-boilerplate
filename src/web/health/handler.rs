use super::service::Service;
use crate::web::errors::ServiceError;
use actix_web::{get, web, HttpResponse};

#[get("/health")]
async fn health(rds: web::Data<redis::Client>) -> Result<HttpResponse, ServiceError> {
    Service::health(&rds).await?;

    Ok(HttpResponse::Ok().body(format!("ok!")))
}
