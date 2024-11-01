use super::service::Service;
use crate::AppState;
use actix_web::{get, web, Error, HttpResponse};

#[get("/health")]
async fn health(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    Service::health(&app_state.redis).await?;

    Ok(HttpResponse::Ok().body(format!("ok!")))
}
