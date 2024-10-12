use actix_web::web;
pub mod db;
pub mod error;
pub mod health;
pub mod index;
pub mod middleware;
pub mod user;

pub struct AppState {
    pub app_name: String,
}

pub fn config_app(cfg: &mut web::ServiceConfig) {
    index::routes::init_routes(cfg);
    health::routes::init_routes(cfg);
}

pub fn config_api(cfg: &mut web::ServiceConfig) {
    user::routes::init_routes(cfg);
}
