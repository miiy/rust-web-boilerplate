use super::auth;
use super::post;
use super::user;
use actix_web::web;

pub fn config_api(cfg: &mut web::ServiceConfig) {
    user::route::init_routes(cfg);
    post::route::init_routes(cfg);
    auth::route::init_routes(cfg);
}
