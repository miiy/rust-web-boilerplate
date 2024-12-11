use super::about;
use super::auth;
use super::health;
use super::index;
use super::post;
use actix_web::web;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    index::route::init_routes(cfg);
    about::route::init_routes(cfg);
    health::route::init_routes(cfg);
    post::route::init_routes(cfg);
    auth::route::init_routes(cfg);
}
