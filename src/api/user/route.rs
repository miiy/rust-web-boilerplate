use super::handler;
use actix_web::web;
use crate::api::middleware::authorization;

// GET /users
// GET /users/{id}
// POST /users
// PUT /users/{id}
// DELETE /users/{id}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // use actix_web_httpauth::middleware::HttpAuthentication;
    // use crate::api::middleware::authorization;
    // let auth = HttpAuthentication::with_fn(middleware::auth::validator);
    // .wrap(auth)

    let auth_middleware = authorization::Authorization;
    cfg.service(
        web::scope("/v1/users").wrap(auth_middleware).service(web::resource("/me").route(web::get().to(handler::me))),
    );
}
