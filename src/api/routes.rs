use super::post;
use super::user;
use actix_web::web;

pub fn config_api(cfg: &mut web::ServiceConfig) {
    user::routes::init_routes(cfg);
    post::routes::init_routes(cfg);
}
