use actix_web::web;
use super::user;
use super::post;

pub fn config_api(cfg: &mut web::ServiceConfig) {
    user::routes::init_routes(cfg);
    post::routes::init_routes(cfg);
}
