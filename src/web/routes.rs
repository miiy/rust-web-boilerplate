use super::about;
use super::auth;
use super::health;
use super::index;
use super::post;
use actix_web::web;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    index::routes::init_routes(cfg);
    about::routes::init_routes(cfg);
    health::routes::init_routes(cfg);
    post::routes::init_routes(cfg);
    auth::routes::init_routes(cfg);
}
