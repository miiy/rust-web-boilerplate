use super::auth;
use super::post;
use super::user;
use super::profile;
use crate::api::middleware::authentication;
use actix_web::web;

pub fn config_api(cfg: &mut web::ServiceConfig) {
    let auth_middleware = authentication::Authentication;

    user::route::init_routes(cfg);
    post::route::init_routes(cfg);
    profile::route::init_routes(cfg, auth_middleware);
    auth::route::init_routes(cfg);
}
