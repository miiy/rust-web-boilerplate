use super::handler;
use crate::api::middleware::authentication;
use actix_web::web;

// GET /profile
pub fn init_routes(cfg: &mut web::ServiceConfig, auth_middleware: authentication::Authentication) {
    // use actix_web_httpauth::middleware::HttpAuthentication;
    // use crate::api::middleware::authorization;
    // let auth = HttpAuthentication::with_fn(middleware::auth::validator);
    // .wrap(auth)

    cfg.service(
        web::scope("/v1/profile")
            .wrap(auth_middleware)
            .service(web::resource("").route(web::get().to(handler::profile))),
    );
}
