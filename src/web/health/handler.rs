use super::service::Service;
use crate::web::errors::ServiceError;
use crate::AppState;
use actix_web::{get, web, HttpResponse};

#[get("/health")]
async fn health(app_state: web::Data<AppState>) -> Result<HttpResponse, ServiceError> {
    Service::health(&app_state.redis).await?;

    Ok(HttpResponse::Ok().body(format!("ok!")))
}
