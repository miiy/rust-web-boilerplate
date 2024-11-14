use super::service::Service;
use crate::api::error::APIError;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};

// GET /users/me
pub async fn me(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let id = 1;
    let resp = Service::detail(id, &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}
