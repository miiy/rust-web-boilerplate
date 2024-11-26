use super::service::Service;
use crate::api::error::APIError;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};

// GET /users/{username}
pub async fn user(
    username: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let resp = Service::user(&username, &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}
