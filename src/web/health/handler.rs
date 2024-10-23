use actix_web::{get, web, Error, HttpResponse};
use super::service::Service;

#[get("/health")]
async fn health(rds: web::Data<redis::Client>) -> Result<HttpResponse, Error> {
    let _ = Service::health(&rds).await.map_err(|e| {
        log::error!("{e}");
    });

    Ok(HttpResponse::Ok().body(format!("ok!")))
}
