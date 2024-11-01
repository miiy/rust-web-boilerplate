use crate::AppState;
use actix_web::{web, Error, HttpResponse};
use askama::Template;

// GET /register
pub async fn register(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let t = super::service::register(app_state).await;
    Ok(HttpResponse::Ok().body(t.render().unwrap()))
}

// GET /login
pub async fn login(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let t = super::service::login(app_state).await;
    Ok(HttpResponse::Ok().body(t.render().unwrap()))
}

// GET /logout
pub async fn logout(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let _resp = super::service::logout(app_state).await.unwrap();
    Ok(HttpResponse::Ok().body(""))
}
