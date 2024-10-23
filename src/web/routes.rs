use actix_web::web;
use super::index;
use super::about;
use super::health;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    index::routes::init_routes(cfg);
    about::routes::init_routes(cfg);
    health::routes::init_routes(cfg);
}
